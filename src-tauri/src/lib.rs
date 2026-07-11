//! FLA Dynoview — Tauri backend.
//!
//! Thin command layer over [`fladyno_core`]: it reads Bosch FLA 203 floppy
//! images and `.ERG` run files and hands decoded, serializable DTOs to the
//! Svelte frontend. Filesystem-touching commands are `async` so Tauri runs them
//! off the webview thread.

mod commands;
mod db;
mod error;
mod model;
mod reset;
mod settings;

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
            commands::import_runs,
            commands::import_all,
            commands::list_db_runs,
            commands::get_db_run,
            commands::update_run_description,
            commands::update_run_date,
            commands::update_run_overrides,
            commands::delete_db_run,
            commands::read_image_data_uri,
            commands::reset_image,
            commands::initial_path,
            commands::app_paths,
            commands::open_data_folder,
            commands::get_settings,
            commands::set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FLA Dynoview");
}
