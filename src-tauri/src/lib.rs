pub mod gateway;
pub mod repository;

#[macro_use]
pub mod service;

use crate::{repository::{database_repository::Database, storage_repository::StorageRepository}};
use service::{database_service::*, notification_service::*, storage_service::*};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent}};

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
    let close = MenuItem::with_id(&app_handle, "close", "close", true, None::<&str>).unwrap();

    let menu = Menu::with_items(&app_handle, &[&close]).unwrap();

    let cloned = app_handle.clone();
    let _ = TrayIconBuilder::with_id("tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |_tray, event| match event.id.as_ref() {
            "close" => {
                close_app(cloned.clone());
            },
            _ => {

            }
        })
        .on_tray_icon_event(|tray: &TrayIcon, event| match event {
            TrayIconEvent::Click { 
                id: _, 
                position: _, 
                rect: _, 
                button: MouseButton::Left, 
                button_state: MouseButtonState::Up
            } => {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = tray.set_visible(false);
                }
            },
            _ => {}
        })
        .icon(app_handle.default_window_icon().unwrap().clone())
        .build(&app_handle)
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
            initialize_database,
            get_record,
            add_record,
            get_all_records,
            delete_record,
            send_notification,
            notification_loop,
            store_storage,
            get_storage,
            tray_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
