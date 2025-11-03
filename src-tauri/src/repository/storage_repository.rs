use std::fs::{self, File};

use serde_json::json;
use shared::errors::storage_error::StorageError;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

#[derive(Clone)]
pub struct StorageRepository {
    pub path: String,
}

impl StorageRepository {
    pub fn new(app: AppHandle) -> Self {
        let mut dir = app.path().app_data_dir().unwrap();
        dir.push("storage");

        if !fs::exists(&dir).unwrap() {
            fs::create_dir(&dir).unwrap();
            dir.push("storage.json");
            File::create(&dir).unwrap();
        } else {
            dir.push("storage.json");
        }

        StorageRepository {
            path: dir.to_str().unwrap().to_string(),
        }
    }

    pub fn store(&self, app: AppHandle, key: String, value: String) -> Result<(), StorageError> {
        let storage = app.store(self.path.clone());
        if let Ok(store) = storage {
            println!("saving {} at {}", value, key);
            store.set(&key, json!({"value": value}));
            println!("saved {} at {}", value, key);
            if let Err(_) = store.save() {
                return Err(StorageError::storage_value_set_error(value));
            }
            store.close_resource();
            println!("closing...");
            Ok(())
        } else {
            Err(StorageError::storage_access_error())
        }
    }

    pub fn get(&self, app: AppHandle, key: String) -> Result<String, StorageError> {
        let storage = app.store(self.path.clone());
        if let Ok(store) = storage {
            let value = store.get(&key);
            if let Some(value) = value {
                store.close_resource();
                Ok(value.to_string())
            } else {
                store.close_resource();
                Err(StorageError::storage_value_access_error(key))
            }
        } else {
            Err(StorageError {
                message: "error accessing storage".to_string(),
            })
        }
    }
}
