use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::Properties;
use macros::AutoNew;

#[derive(Properties, PartialEq, Clone, Deserialize, Default, Serialize, Debug, AutoNew)]
pub struct Record{
    pub uuid: String,
    pub name: String,
    pub date: String,
    pub time: String
}

impl Record {
    pub fn record_by_name(name: String) -> Record {
        Record{uuid: Uuid::nil().to_string(), name, date: "".to_string(), time: "".to_string() }
    }
}