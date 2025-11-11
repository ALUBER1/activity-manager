pub struct FormError {
    pub field: String,
    pub error: ErrorReason
}

pub enum ErrorReason {
    Format(String), Empty, Past, Fallback(String)
}
