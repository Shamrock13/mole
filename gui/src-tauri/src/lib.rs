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

use tauri::Manager;

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

/// Status metrics for the front-end. The bundled collector makes the
/// DMG self-contained; a system-wide `mo` install is the fallback so
/// CLI users get identical data.
#[tauri::command]
fn mole_status() -> Result<String, String> {
    if let Some(bin) = bundled_status() {
        match run_capture(bin.as_os_str(), &["--json"]) {
            Ok(out) => return Ok(out),
            Err(_) => {} // fall through to a system install
        }
    }
    run_capture(OsStr::new("mo"), &["status", "--json"])
        .or_else(|_| run_capture(OsStr::new("/usr/local/bin/mo"), &["status", "--json"]))
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mole_status, set_fan_mode])
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

            let _ = window;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Mole GUI");
}
