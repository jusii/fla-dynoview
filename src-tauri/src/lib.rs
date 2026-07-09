//! FLA Dynoview — Tauri backend.
//!
//! Thin command layer over [`fladyno_core`]: it reads Bosch FLA 203 floppy
//! images and `.ERG` run files and hands decoded, serializable DTOs to the
//! Svelte frontend. Filesystem-touching commands are `async` so Tauri runs them
//! off the webview thread.

mod commands;
mod error;
mod model;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::open_image,
            commands::open_erg_file,
            commands::read_erg_from_image,
            commands::parse_erg,
            commands::get_shop_header,
            commands::initial_path,
            commands::app_paths,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FLA Dynoview");
}
