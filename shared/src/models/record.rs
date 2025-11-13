use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use macros::AutoNew;
use uuid::Uuid;
use yew::Properties;

use crate::models::dto::record_dto::RecordDto;

#[derive(Properties, PartialEq, Clone, Default, Debug, AutoNew)]
pub struct Record {
    pub uuid: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub notified_at: NaiveDateTime,
}

impl Record {
    pub fn record_by_name(name: String) -> Record {
        Record {
            uuid: Uuid::nil(),
            name,
            date: NaiveDate::default(),
            time: NaiveTime::default(),
            notified_at: NaiveDateTime::default(),
        }
    }

    pub fn get_date(&self) -> String {
        self.date.format("%d/%m/%Y").to_string()
    }

    pub fn get_time(&self) -> String {
        self.time.format("%M:%H").to_string()
    }

    pub fn get_notified(&self) -> String {
        self.notified_at.format("%d/%m/%Y,%H:%M").to_string()
    }

    pub fn from(record: RecordDto) -> Self {
        Record {
            uuid: Uuid::from_str(&record.uuid).unwrap(),
            name: record.name,
            date: record.date,
            time: record.time,
            notified_at: NaiveDateTime::parse_from_str(&record.notified_at, "%d/%m/%Y,%H:%M").unwrap(),
        }
    }

    pub fn from_vec(records: Vec<RecordDto>) -> Vec<Self> {
        let mut vec = Vec::<Record>::new();
        for record in records {
            vec.push(Record {
                uuid: Uuid::from_str(&record.uuid).unwrap(),
                name: record.name,
                date: record.date,
                time: record.time,
                notified_at: NaiveDateTime::parse_from_str(&record.notified_at, "%d/%m/%Y,%H:%M").unwrap(),
            });
        }
        vec
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
