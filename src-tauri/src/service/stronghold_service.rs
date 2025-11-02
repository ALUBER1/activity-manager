use std::sync::Mutex;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use shared::{models::storage_entry::StorageEntry, utils::normalize::NormalizeDelay};
use tauri::{AppHandle, State};

use crate::{repository::storage_repository::StorageRepository, service::storage_service::{get_storage, store_storage}};

fn encrypt(storage_entry: StorageEntry) -> String {

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let enc = argon2
        .hash_password(storage_entry.value.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();
    enc
}

#[tauri::command]
pub fn verify(storage_entry: StorageEntry, app: AppHandle, state: State<'_, Mutex<Option<StorageRepository>>>) -> StorageEntry {
    println!("password to test: {}", storage_entry.value);
    let argon2 = Argon2::default();

    let entry = get_storage(app, storage_entry.clone(), state);
    
    println!("password: {}", entry.value);

    let binding = NormalizeDelay::normalize_color(entry.value);
    let parsed = match PasswordHash::new(&binding) {
        Ok(p) => p,
        Err(_) => {println!("problem");return StorageEntry::default();}
    };
    println!("is ok: {}", argon2.verify_password(storage_entry.value.as_bytes(), &parsed).is_ok().to_string());
    StorageEntry::new(storage_entry.key, argon2.verify_password(storage_entry.value.as_bytes(), &parsed).is_ok().to_string())
}

#[tauri::command]
pub fn store_password(storage_entry: StorageEntry, app: AppHandle, state: State<'_, Mutex<Option<StorageRepository>>>) -> StorageEntry {
    store_storage(app, StorageEntry::new(storage_entry.key.clone(), encrypt(storage_entry.clone())), state);
    storage_entry
}