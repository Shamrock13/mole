//! Tauri shell for the Mole GUI.
//!
//! The Rust layer stays deliberately thin: it shells out to Mole's
//! existing, audited binaries (`mo status --json`) instead of
//! reimplementing collection or cleanup logic. All destructive flows
//! remain in the shell/Go core where the safety contract
//! (mole_delete, should_protect_path, operation logs) lives.

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::Duration;

use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, PhysicalPosition, WindowEvent};

const TRAY_ID: &str = "mole-tray";
const TRAY_TITLE_INTERVAL: Duration = Duration::from_secs(10);

/// Menu bar configuration, driven by the Settings UI and persisted on
/// the front-end (localStorage). `display` is "icon" or "metrics".
struct TrayState {
    enabled: bool,
    show_metrics: bool,
}

impl Default for TrayState {
    fn default() -> Self {
        Self {
            enabled: true,
            show_metrics: false,
        }
    }
}

fn run_capture(program: &OsStr, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("failed to launch {}: {e}", program.to_string_lossy()))?;

    if !output.status.success() {
        return Err(format!(
            "{} exited with {}",
            program.to_string_lossy(),
            output.status.code().unwrap_or(-1)
        ));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("invalid utf8: {e}"))
}

/// The Go status collector bundled next to the app executable
/// (Contents/MacOS/mole-status), shipped via Tauri's externalBin.
fn bundled_status() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let path = exe.parent()?.join("mole-status");
    path.is_file().then_some(path)
}

/// Status metrics. The bundled collector makes the DMG self-contained;
/// a system-wide `mo` install is the fallback so CLI users get
/// identical data. `--fast` skips collectors the GUI never reads
/// (Bluetooth, Trash size, proxy), which cost seconds per cold run.
fn collect_status_json() -> Result<String, String> {
    if let Some(bin) = bundled_status() {
        if let Ok(out) = run_capture(bin.as_os_str(), &["--json", "--fast"]) {
            return Ok(out);
        }
        // Fall through to a system install.
    }
    run_capture(OsStr::new("mo"), &["status", "--json"])
        .or_else(|_| run_capture(OsStr::new("/opt/homebrew/bin/mo"), &["status", "--json"]))
        .or_else(|_| run_capture(OsStr::new("/usr/local/bin/mo"), &["status", "--json"]))
}

/// Async so collection runs off the main thread: a sync Tauri command
/// executes on the UI thread, and the collector takes seconds per
/// sample. That blocked the whole app behind a beachball.
#[tauri::command]
async fn mole_status() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(collect_status_json)
        .await
        .map_err(|e| format!("status task failed: {e}"))?
}

/// Fan mode selection. Validated against a fixed allowlist; actual SMC
/// control requires a privileged helper and is intentionally a no-op
/// until that helper ships, so the UI state never lies about safety.
#[tauri::command]
fn set_fan_mode(mode: String) -> Result<String, String> {
    match mode.as_str() {
        "auto" | "cool" | "quiet" => Ok(mode),
        other => Err(format!("unsupported fan mode: {other}")),
    }
}

#[tauri::command]
fn set_tray_config(
    app: AppHandle,
    state: tauri::State<'_, Mutex<TrayState>>,
    enabled: bool,
    display: String,
) -> Result<(), String> {
    let show_metrics = display == "metrics";
    {
        let mut tray_state = state.lock().map_err(|e| e.to_string())?;
        tray_state.enabled = enabled;
        tray_state.show_metrics = show_metrics;
    }
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_visible(enabled);
        if !enabled || !show_metrics {
            let _ = tray.set_title(None::<&str>);
        }
    }
    if !enabled {
        if let Some(panel) = app.get_webview_window("tray") {
            let _ = panel.hide();
        }
    }
    Ok(())
}

#[tauri::command]
fn show_main_window(app: AppHandle) {
    if let Some(panel) = app.get_webview_window("tray") {
        let _ = panel.hide();
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

fn rect_to_physical(rect: &tauri::Rect, scale: f64) -> (f64, f64, f64, f64) {
    let (x, y) = match rect.position {
        tauri::Position::Physical(p) => (p.x as f64, p.y as f64),
        tauri::Position::Logical(p) => (p.x * scale, p.y * scale),
    };
    let (w, h) = match rect.size {
        tauri::Size::Physical(s) => (s.width as f64, s.height as f64),
        tauri::Size::Logical(s) => (s.width * scale, s.height * scale),
    };
    (x, y, w, h)
}

/// Left-clicking the tray icon toggles the popover panel, anchored
/// just below the menu bar item.
fn toggle_tray_panel(app: &AppHandle, rect: tauri::Rect) {
    let Some(panel) = app.get_webview_window("tray") else {
        return;
    };
    if panel.is_visible().unwrap_or(false) {
        let _ = panel.hide();
        return;
    }

    let scale = panel.scale_factor().unwrap_or(1.0);
    let (icon_x, icon_y, icon_w, icon_h) = rect_to_physical(&rect, scale);
    let panel_w = panel
        .outer_size()
        .map(|s| s.width as f64)
        .unwrap_or(380.0 * scale);
    let x = (icon_x + icon_w / 2.0 - panel_w / 2.0).max(8.0 * scale);
    let y = icon_y + icon_h + 6.0 * scale;
    let _ = panel.set_position(PhysicalPosition::new(x, y));
    let _ = panel.show();
    let _ = panel.set_focus();
}

/// Background refresher for the menu bar title (e.g. "37%"). Only does
/// work while the metrics display mode is on; the default icon-only
/// mode costs nothing.
fn spawn_tray_title_updater(app: AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(TRAY_TITLE_INTERVAL);
        let wants_metrics = {
            let state = app.state::<Mutex<TrayState>>();
            let guard = match state.lock() {
                Ok(g) => g,
                Err(_) => continue,
            };
            guard.enabled && guard.show_metrics
        };
        if !wants_metrics {
            continue;
        }
        let Some(tray) = app.tray_by_id(TRAY_ID) else {
            continue;
        };
        let title = collect_status_json()
            .ok()
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
            .and_then(|v| v.pointer("/cpu/usage").and_then(|u| u.as_f64()))
            .map(|usage| format!("{}%", usage.round() as i64));
        if let Some(title) = title {
            let _ = tray.set_title(Some(title));
        }
    });
}

fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let open = MenuItemBuilder::with_id("open", "Open Mole").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit Mole").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&open, &quit]).build()?;

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(app.default_window_icon().expect("bundle icon").clone())
        .tooltip("Mole")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open" => show_main_window(app.clone()),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect,
                ..
            } = event
            {
                toggle_tray_panel(tray.app_handle(), rect);
            }
        })
        .build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(TrayState::default()))
        .invoke_handler(tauri::generate_handler![
            mole_status,
            set_fan_mode,
            set_tray_config,
            show_main_window,
            quit_app
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").expect("main window");

            // Native NSVisualEffectView behind the transparent webview:
            // this is what makes the glass panels sample the desktop.
            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(
                &window,
                window_vibrancy::NSVisualEffectMaterial::UnderWindowBackground,
                None,
                None,
            )
            .expect("vibrancy is supported on macOS 10.14+");

            // Closing the main window hides it while the menu bar item
            // is on, so the monitor keeps running like a status app.
            let close_handle = app.handle().clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    let tray_on = close_handle
                        .state::<Mutex<TrayState>>()
                        .lock()
                        .map(|s| s.enabled)
                        .unwrap_or(false);
                    if tray_on {
                        api.prevent_close();
                        if let Some(main) = close_handle.get_webview_window("main") {
                            let _ = main.hide();
                        }
                    }
                }
            });

            setup_tray(app)?;

            if let Some(panel) = app.get_webview_window("tray") {
                #[cfg(target_os = "macos")]
                window_vibrancy::apply_vibrancy(
                    &panel,
                    window_vibrancy::NSVisualEffectMaterial::Popover,
                    None,
                    Some(14.0),
                )
                .expect("vibrancy is supported on macOS 10.14+");

                // The popover behaves like a system menu: it hides as
                // soon as it loses focus.
                let panel_handle = panel.clone();
                panel.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        let _ = panel_handle.hide();
                    }
                });
            }

            spawn_tray_title_updater(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Mole GUI");
}
