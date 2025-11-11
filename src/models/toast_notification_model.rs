use std::cell::Cell;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct ToastNotificationModel {
    pub id: usize,
    pub title: String,
    pub message: String
}

impl ToastNotificationModel {
    pub fn incorrect_field(field: &str) -> Self {
        ToastNotificationModel { 
            id: next_id(), 
            title: format!("incorrect field {}", field), 
            message: match field {
                "date" => String::from("date should have format dd/mm/aaaa"),
                "time" => String::from("time should have format hh:mm"),
                "name" => String::from("please fill all form fields"),
                _ => String::from("error")
            }
        }
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