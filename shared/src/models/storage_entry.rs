use macros::AutoNew;
use serde::{Deserialize, Serialize};

#[derive(AutoNew, Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct StorageEntry {
    pub key: String,
    pub value: String,
}

impl StorageEntry {
    pub fn new_delay(value: String) -> Self {
        StorageEntry {
            key: "delay".to_string(),
            value,
        }
    }

    pub fn new_color(value: String, setting: String) -> Self {
        StorageEntry {
            key: setting,
            value,
        }
    }
}
