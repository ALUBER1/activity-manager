use std::{sync::Mutex, thread, time};

use chrono::{Duration, NaiveDate, Utc};
use shared::{errors::notification_error::NotificationError, models::notification::Notification};
use tauri::{AppHandle, Manager, State};
use crate::{gateway::notifications_gateway::NotificationGateway, repository::database_repository::Database, service::database_service::get_all_records};

#[tauri::command]
pub fn send_notification(app: AppHandle, notification: Notification) -> Result<(), NotificationError> {
    match NotificationGateway::send_notification(&app, notification) {
        Ok(()) => Ok(()),
        Err(e) => {println!("error sending notification");Err(NotificationError{message: e.to_string()})}
    }
}

#[tauri::command]
pub fn notification_loop(app: AppHandle) {
    thread::spawn(move ||{
        loop{
            let state: State<'_, Mutex<Option<Database>>> = app.state();
            let mut db = {
                let guard = state.lock().unwrap();
                guard
            };

            if db.is_none() {
                println!("database doesn't exist yet");
                ()
            } else {
                if let Some(ref mut db) = *db {
                    match db.get_all_records() {
                        Ok(records) => {
                            for mut record in records {
                                if !record.notified && NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap() - Utc::now().date_naive() == Duration::days(1) {
                                    let _ = send_notification(app.clone(), Notification { title: "test".to_string(), body: record.to_string() });
                                    record.notified = true;
                                    let _ = db.update_record(record);
                                }
                            }
                        },
                        Err(e) => {
                            println!("{:?}", e);
                            ()
                        }
                    }
                } else {
                    println!("database error");
                    ()
                }
            }
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
    ()
}