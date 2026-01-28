mod commands;
mod db;
mod models;

use commands::*;
use tauri::menu::{Menu, MenuItem, Submenu, PredefinedMenuItem};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            let file_menu = Submenu::with_items(
                &app_handle,
                "File",
                true,
                &[&PredefinedMenuItem::quit(&app_handle, Some("Quit"))?],
            )?;

            let license_item =
                MenuItem::with_id(&app_handle, "license", "License", true, None::<&str>)?;
            let help_menu =
                Submenu::with_items(&app_handle, "Help", true, &[&license_item])?;

            let menu = Menu::with_items(&app_handle, &[&file_menu, &help_menu])?;
            app.set_menu(menu)?;

            app.on_menu_event(|_app, event| {
                if event.id.as_ref() == "license" {
                    let _ = open::that(
                        "https://github.com/AlbertArakelyan/OptionLedger/blob/main/LICENSE",
                    );
                }
            });

            db::initialize_db(&app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_user,
            list_users,
            delete_user,
            create_option,
            list_options,
            delete_option,
            set_ownership,
            get_ownerships,
            get_matrix_view
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
