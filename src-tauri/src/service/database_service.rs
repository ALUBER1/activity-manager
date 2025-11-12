use crate::{models::record::Record, repository::database_repository::Database};
use shared::models::dto::record_dto::RecordDto;
use std::sync::Mutex;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn create_database(app: AppHandle, state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().unwrap();
    if db.is_none() {
        match Database::new(app) {
            Ok(database) => {
                *db = Some(database);
                println!("db opened correctly")
            }
            Err(e) => println!("{:?}", e),
        }
    } else {
        println!("db already exists");
    }
}

#[tauri::command]
pub fn add_record(record: RecordDto, state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none() {
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut database) = *db {
            match database.add_record(Record::from(record)) {
                Ok(r) => println!("record inserted: {:?}", r),
                Err(e) => println!("{:?}", e),
            }
        } else {
            println!("database error");
        }
    }
}

#[tauri::command]
pub fn delete_record(record: RecordDto, state: State<'_, Mutex<Option<Database>>>) {
    let mut db: std::sync::MutexGuard<'_, Option<Database>> =
        state.lock().expect("error unpacking mutex");
    if db.is_none() {
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut database) = *db {
            match database.delete_record(Record::from(record)) {
                Ok(rec) => println!("deleted record: {:?}", rec),
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        } else {
            println!("database error");
        }
    }
}

#[tauri::command]
pub fn get_all_records(state: State<'_, Mutex<Option<Database>>>) -> Vec<Record> {
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none() {
        println!("database doesn't exist yet");
        Vec::new()
    } else {
        if let Some(ref mut db) = *db {
            match db.get_all_records() {
                Ok(records) => records,
                Err(e) => {
                    println!("{:?}", e);
                    Vec::new()
                }
            }
        } else {
            println!("database error");
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn update_record(record: RecordDto, state: State<'_, Mutex<Option<Database>>>) {
    let mut db = state.lock().expect("error unpacking mutex");
    if db.is_none() {
        println!("database doesn't exist yet");
    } else {
        if let Some(ref mut db) = *db {
            match db.update_record(Record::from(record)) {
                Ok(_) => (),
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        } else {
            println!("database error");
        }
    }
}
