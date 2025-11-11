use std::cell::Cell;

use chrono::{Local, NaiveTime};

use crate::errors::{form_error::{FormError, FormErrorReason}, setting_error::{SettingError, SettingErrorReason}};

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
                FormErrorReason::Empty => format!("please fill form field {}", error.field),
                FormErrorReason::Past => String::from("please insert a future date time"),
                FormErrorReason::Format(format) => format!("{} should have format {}", error.field, format),
                FormErrorReason::Fallback(value) => format!("problem with value {}", value)
            },
            created_at: Local::now().time()
        }
    }

    pub fn incorrect_setting(error: SettingError) -> Self {
        let id = next_id();
        ToastNotificationModel { 
            id: id, 
            title: format!("incorrect field {}", error.field), 
            message: match error.error {
                SettingErrorReason::Empty => format!("please fill form field {}", error.field),
                SettingErrorReason::Format(format) => format!("{} should have format {}", error.field, format),
                SettingErrorReason::NonExistent => format!("please insert a valid input")
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