#[derive(PartialEq, Clone, Debug, Default)]
pub struct ToastNotificationModel {
    pub id: usize,
    pub title: String,
    pub message: String
}

impl ToastNotificationModel {
    pub fn incorrect_field(field: &str) -> Self {
        ToastNotificationModel { 
            id: 0, 
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