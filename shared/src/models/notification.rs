use macros::AutoNew;
use serde::{Deserialize, Serialize};

#[derive(AutoNew, Serialize, Deserialize)]
pub struct Notification {
    pub title: String,
    pub body: String
}