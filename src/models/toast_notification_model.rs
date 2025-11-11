use std::cell::Cell;

use chrono::{Local, NaiveTime};

#[derive(Clone, Debug, Default)]
pub struct ToastNotificationModel {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub created_at: NaiveTime
}

impl ToastNotificationModel {
    pub fn incorrect_field(field: &str) -> Self {
        let id = next_id();
        ToastNotificationModel { 
            id: id, 
            title: format!("incorrect field {}", field), 
            message: match field {
                "date" => String::from(format!("date should have format dd/mm/aaaa {}", id)),
                "time" => String::from(format!("time should have format hh:mm {}", id)),
                "name" => String::from(format!("please fill all form fields {}", id)),
                _ => String::from("error")
            },
            created_at: Local::now().time()
        }
    }
}

impl PartialEq for ToastNotificationModel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

thread_local! {
    static ID: Cell<usize> = Cell::new(0);
}

fn next_id() -> usize {
    ID.set(ID.get() + 1);

    if ID.get() == 1 {
        0
    } else {
        ID.get() - 1
    }
}