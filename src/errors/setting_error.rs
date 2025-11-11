pub struct SettingError {
    pub field: String,
    pub error: SettingErrorReason,
}

pub enum SettingErrorReason {
    Format(String),
    NonExistent,
    Empty,
}
