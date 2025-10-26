use macros::AutoNew;
use serde::{Deserialize, Serialize};

#[derive(AutoNew, Serialize, Deserialize)]
pub struct StorageEntry {
    pub key: String,
    pub value: String
}