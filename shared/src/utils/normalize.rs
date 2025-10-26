pub struct NormalizeDelay;

impl NormalizeDelay {
    pub fn normalize(value: String) -> i64 {
        value.split(":")
            .nth(1)
            .unwrap()
            .replace("\"", "")
            .replace("}", "")
            .parse::<i64>()
            .unwrap()
    }
}