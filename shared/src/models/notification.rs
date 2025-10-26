use macros::AutoNew;
use serde::{Deserialize, Serialize};

use crate::models::record::Record;

#[derive(AutoNew, Serialize, Deserialize)]
pub struct Notification {
    pub title: String,
    pub body: String
}

impl Notification {
    pub fn event_due(record: Record) -> Self {
        Notification{title: "event due".to_string(), body: format!("event: {} due at {} day {}", record.name, record.time, record.date)}
    }
}