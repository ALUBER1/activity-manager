use std::fmt::Display;

use crate::schema::schema::events;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use serde::Serialize;
use shared::models::dto::record_dto::RecordDto;

#[derive(Clone, Default, Debug, Queryable, Selectable, Insertable, AsChangeset, Serialize)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub uuid: String,
    pub name: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub notified_at: String,
}

impl Record {
    pub fn from(record: RecordDto) -> Self {
        Record {
            uuid: record.uuid.to_string(),
            name: record.name,
            date: record.date,
            time: record.time,
            notified_at: if record.notified_at.eq(&NaiveDateTime::default()
                .format("%d/%m/%Y,%H:%M")
                .to_string())
            {
                String::new()
            } else {
                record.notified_at
            },
        }
    }

    pub fn get_date(&self) -> String {
        self.date.format("%d/%m/%Y").to_string()
    }

    pub fn get_time(&self) -> String {
        self.time.format("%H:%M").to_string()
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
