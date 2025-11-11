pub struct FormError {
    pub field: String,
    pub error: FormErrorReason,
}

pub enum FormErrorReason {
    Format(String),
    Empty,
    Past,
    Fallback(String),
}
