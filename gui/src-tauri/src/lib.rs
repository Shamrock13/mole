//! Tauri shell for the Mole GUI.
//!
//! The Rust layer stays deliberately thin: it shells out to Mole's
//! existing, audited binaries (`mo status --json`) instead of
//! reimplementing collection or cleanup logic. All destructive flows
//! remain in the shell/Go core where the safety contract
//! (mole_delete, should_protect_path, operation logs) lives.

use std::process::Command;

use tauri::Manager;

/// Resolve the `mo` binary: PATH first, then the standard install spot.
fn mo_binary() -> &'static str {
    if Command::new("mo").arg("--version").output().is_ok() {
        "mo"
    } else {
        "/usr/local/bin/mo"
    }
}

/// Run `mo status --json` and hand the raw payload to the front-end.
#[tauri::command]
fn mole_status() -> Result<String, String> {
    let output = Command::new(mo_binary())
        .args(["status", "--json"])
        .output()
        .map_err(|e| format!("failed to launch mo: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "mo status exited with {}",
            output.status.code().unwrap_or(-1)
        ));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("invalid utf8 from mo status: {e}"))
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
