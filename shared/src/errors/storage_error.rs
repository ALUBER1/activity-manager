use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageError {
    pub message: String,
}

impl StorageError {
    pub fn storage_access_error() -> Self {
        StorageError {
            message: "error accessing storage".to_string(),
        }
    }

    pub fn storage_value_access_error(value: String) -> Self {
        StorageError {
            message: format!("error accessing key {} in storage", value),
        }
    }

    pub fn storage_value_set_error(value: String) -> Self {
        StorageError {
            message: format!("error setting key {} in storage", value),
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Notification Error: {}", self.message)
    }
}

impl Error for StorageError {
    fn description(&self) -> &str {
        &self.message
    }
}
