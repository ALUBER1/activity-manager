use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::Properties;
use macros::AutoNew;

#[derive(Properties, PartialEq, Clone, Deserialize, Default, Serialize, Debug, AutoNew)]
pub struct Record{
    pub uuid: String,
    pub name: String,
    pub date: String,
    pub time: String,
    pub notified_at: String
}

impl Record {
    pub fn record_by_name(name: String) -> Record {
        Record{uuid: Uuid::nil().to_string(), name, date: "".to_string(), time: "".to_string(), notified_at: "".to_string() }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
