use std::cell::Cell;

use chrono::{Local, NaiveTime};

use crate::errors::form_error::{ErrorReason, FormError};

#[derive(Clone, Debug, Default)]
pub struct ToastNotificationModel {
    pub id: usize,
    pub title: String,
    pub message: String,
    pub created_at: NaiveTime
}

impl ToastNotificationModel {
    pub fn incorrect_field(error: FormError) -> Self {
        let id = next_id();
        ToastNotificationModel { 
            id: id, 
            title: format!("incorrect field {}", error.field), 
            message: match error.error {
                ErrorReason::Empty => format!("please fill form field {}", error.field),
                ErrorReason::Past => String::from("please insert a future date time"),
                ErrorReason::Format(format) => format!("{} should have format {}", error.field, format),
                ErrorReason::Fallback(value) => format!("problem with value: {}", value)
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