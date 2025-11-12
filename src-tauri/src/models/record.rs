use std::fmt::Display;

use crate::schema::schema::events;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use shared::models::dto::record_dto::RecordDto;

#[derive(
    Clone, Default, Debug, Deserialize, Serialize, Queryable, Selectable, Insertable, AsChangeset,
)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub uuid: String,
    pub name: String,
    pub date: String,
    pub time: String,
    pub notified_at: String,
}

impl Record {
    pub fn from(record: RecordDto) -> Self {
        Record {
            uuid: record.uuid,
            name: record.name,
            date: record.date,
            time: record.time,
            notified_at: record.notified_at,
        }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
