use std::sync::Mutex;

use shared::models::storage_entry::StorageEntry;
use tauri::{AppHandle, State};

use crate::repository::storage_repository::StorageRepository;

#[tauri::command]
pub fn store_storage(app: AppHandle, storage_entry: StorageEntry, state: State<'_, Mutex<StorageRepository>>) {
    let temp = state.lock().unwrap();
    let storage = temp.clone();
    drop(temp);

    println!("got storage");
    if let Err(e) = storage.store(app, storage_entry.key, storage_entry.value.clone()) {
        println!("error storing value: {}", e.message);
    } else {
        println!("stored value {}", storage_entry.value);
    }
    
    
}

#[tauri::command]
pub fn get_storage(app: AppHandle, storage_entry: StorageEntry, state: State<'_, Mutex<StorageRepository>>) -> String {
    let temp = state.lock().unwrap();
    let storage = temp.clone();
    drop(temp);

    let result = storage.get(app, storage_entry.key);
    if let Err(e) = result {
        println!("error storing value: {}", e.message);
    } else {
        return result.unwrap();
    }
    
    String::new()
}