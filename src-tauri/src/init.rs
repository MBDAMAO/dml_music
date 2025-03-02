use tauri::{App, Manager};
// #[cfg(target_os = "macos")]
// use window_vibrancy::NSVisualEffectMaterial;
use window_vibrancy::{self};

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let win = app.get_webview_window("main").unwrap();

    // #[cfg(target_os = "macos")]
    // window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI)
    //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    window_vibrancy::apply_acrylic(&win, Some((255, 255, 255, 128)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}
