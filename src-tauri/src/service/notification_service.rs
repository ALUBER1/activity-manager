use std::{sync::Mutex, thread, time};

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use shared::{errors::notification_error::NotificationError, models::{notification::Notification, record::Record}};
use tauri::{AppHandle, Manager, State};
use crate::{gateway::notifications_gateway::NotificationGateway, repository::{database_repository::Database, storage_repository::StorageRepository}};

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
            let database: State<'_, Mutex<Option<Database>>> = app.state();
            let records = {
                let mut guard = database.lock().unwrap();
                if let Some(ref mut db) = *guard {
                    match db.get_all_records() {
                        Ok(records) => records,
                        _ => Vec::new()
                    }
                } else {
                    Vec::new()
                }
            };

            let storage_repository: State<'_, Mutex<StorageRepository>> = app.state();
            let clone = app.clone();
            let delay = {
                let guard = storage_repository.lock().unwrap();
                match (*guard).get(clone, "delay".to_string()) {
                    Ok(value) => value.split(":")
                                            .nth(1)
                                            .unwrap()
                                            .replace("\"", "")
                                            .replace("}", "")
                                            .parse::<u64>()
                                            .unwrap(),
                    Err(_) => 3600_u64
                }
            };

            for mut record in records {
                let date = NaiveDate::parse_from_str(&record.date, "%d/%m/%Y").unwrap();
                let time = NaiveTime::parse_from_str(&record.time, "%H:%M").unwrap();
                let date_time = NaiveDateTime::new(date, time);
                let now = Local::now().naive_local();
                let diff = (date_time - now).num_seconds();
                if record.notified_at.is_empty() && (diff > (delay - 60_u64).try_into().unwrap() && diff < delay.try_into().unwrap()) {
                    let _ = send_notification(app.clone(), Notification { title: record.name.clone(), body: format_notification_body(&record) });
                    record.notified_at = Local::now().format("%d/%m/%Y,%H:%M").to_string();
                    let _ = {
                        let mut guard = database.lock().unwrap();
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