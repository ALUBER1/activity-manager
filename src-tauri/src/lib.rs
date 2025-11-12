pub mod gateway;
pub mod repository;
pub mod schema;
pub mod models;

#[macro_use]
pub mod service;

use crate::repository::{database_repository::Database, storage_repository::StorageRepository};
use rand::RngCore;
use service::{
    database_service::*, notification_service::*, storage_service::*, stronghold_service::*,
};
use std::{fs, io::Write, sync::Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

#[tauri::command]
fn close_app(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.close();
    }
}

#[tauri::command]
fn minimize_app(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.minimize();
    }
}

#[tauri::command]
fn maximize_app(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        if window.is_maximized().expect("error") {
            let _ = window.unmaximize();
        } else {
            let _ = window.maximize();
        }
    }
}

#[tauri::command]
fn tray_app(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
    app_handle
        .tray_by_id("tray")
        .unwrap()
        .set_visible(true)
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notifications::init())
        .manage(Mutex::new(None::<Database>))
        .manage(Mutex::new(None::<StorageRepository>))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_record,
            maximize_app,
            minimize_app,
            close_app,
            create_database,
            add_record,
            get_all_records,
            delete_record,
            send_notification,
            notification_loop,
            store_storage,
            get_storage,
            tray_app,
            verify,
            store_password
        ])
        .setup(|app| {
            let salt_path = app
                .path()
                .app_data_dir()
                .expect("could not resolve app data path")
                .join("secrets\\salt.txt");
            if let Some(parent_dir) = salt_path.parent() {
                fs::create_dir_all(parent_dir)?;
            }
            if !salt_path.exists() {
                let mut salt = [0u8; 32];
                rand::rng().fill_bytes(&mut salt);
                let mut file = fs::File::create(&salt_path)?;
                file.write_all(&salt)?;
            }

            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            let app_handle = app.app_handle();
            let close =
                MenuItem::with_id(app_handle, "close", "close", true, None::<&str>).unwrap();

            let menu = Menu::with_items(app_handle, &[&close]).unwrap();

            let cloned = app_handle.clone();
            let tray = TrayIconBuilder::with_id("tray")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |_tray, event| match event.id.as_ref() {
                    "close" => {
                        close_app(cloned.clone());
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray: &TrayIcon, event| match event {
                    TrayIconEvent::Click {
                        id: _,
                        position: _,
                        rect: _,
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = tray.set_visible(false);
                        }
                    }
                    _ => {}
                })
                .icon(app_handle.default_window_icon().unwrap().clone())
                .build(app_handle)
                .unwrap();
            tray.set_visible(false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
