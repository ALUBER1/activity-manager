use std::fmt::Display;

use macros::AutoNew;
use serde::{Deserialize, Serialize};

use crate::models::record::Record;

#[derive(Clone, Deserialize, Default, Serialize, Debug, AutoNew)]
pub struct RecordDto {
    pub uuid: String,
    pub name: String,
    pub date: String,
    pub time: String,
    pub notified_at: String,
}

impl RecordDto {
    pub fn from(record: Record) -> Self {
        RecordDto {
            uuid: record.uuid,
            name: record.name,
            date: record.date,
            time: record.time,
            notified_at: record.notified_at,
        }
    }
}

impl Display for RecordDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
