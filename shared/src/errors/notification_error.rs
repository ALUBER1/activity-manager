use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationError {
    pub message: String,
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Notification Error: {}", self.message)
    }
}

impl Error for NotificationError {
    fn description(&self) -> &str {
        &self.message
    }
}
