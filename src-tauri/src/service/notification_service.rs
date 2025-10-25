use std::{sync::Mutex, thread, time};

use chrono::{Duration, NaiveDate, Utc};
use shared::{errors::notification_error::NotificationError, models::{notification::Notification, record::Record}};
use tauri::{AppHandle, Manager, State};
use crate::{gateway::notifications_gateway::NotificationGateway, repository::database_repository::Database};

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
            let records = {
                let mut guard = state.lock().unwrap();
                if let Some(ref mut db) = *guard {
                    match db.get_all_records() {
                        Ok(records) => records,
                        _ => Vec::new()
                    }
                } else {
                    Vec::new()
                }
            };

            for mut record in records {
                if !record.notified && NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap() - Utc::now().date_naive() == Duration::days(1) {
                    let _ = send_notification(app.clone(), Notification { title: "test".to_string(), body: format_notification_body(&record) });
                    record.notified = true;
                    let _ = {
                        let mut guard = state.lock().unwrap();
                        if let Some(ref mut db) = *guard {
                            match db.update_record(record) {
                                Ok(_) => (),
                                _ => ()
                            }
                        } else {
                            ()
                        }
                    };
                }
            }           
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
    ()
}

fn format_notification_body(record: &Record) -> String {
    format!("you have {} due at {} the day {}", record.name, record.time, record.date)
}