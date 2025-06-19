#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, PhysicalSize, Size};
use window_shadows::set_shadow;

fn main() {
    let builder = tauri::Builder::default().setup(|app| {
        let window = app.get_window("main").unwrap();
        if cfg!(not(target_os = "macos")) {
            window.set_decorations(false).unwrap();
            window
                .set_size(Size::Physical(PhysicalSize {
                    width: 920,
                    height: 520,
                }))
                .unwrap();
        }
        #[cfg(not(target_os = "linux"))]
        set_shadow(&window, true).unwrap();
        window.show().unwrap();
        Ok(())
    });

    builder
        .run(tauri::generate_context!())
        .unwrap_or(runtime_error())
}

fn runtime_error() {
    let os = std::env::consts::OS;
    let url = format!("https://amcl.armoe.cn/install/{}.html", os);
    open::that(&url).unwrap();
    std::process::exit(0);
}
