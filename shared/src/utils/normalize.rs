use std::time::Duration;

pub struct NormalizeDelay;

impl NormalizeDelay {
    pub fn normalize(value: String) -> i64 {
        value.split(":")
            .collect::<Vec<&str>>()[1]
            .replace("\"", "")
            .replace("}", "")
            .parse::<i64>()
            .unwrap()
    }

    pub fn normalize_color(value: String) -> String {
        if value.is_empty() {return String::new();}
        value.split(":")
            .collect::<Vec<&str>>()[1]
            .replace("\"", "")
            .replace("}", "")
            .replace("\\", "")
    }

    pub fn convert_to_num(input: String) -> String {
        if !input.chars().any(|c|{c.is_alphabetic()}) {
            let mut split = input.split("/");
            if let Some(days) = split.nth(0) {
                let days_num: u64 = days.parse::<u64>().unwrap();
                if let Some(minutes) = split.nth(0) {
                    let minutes_num: u64 = minutes.parse::<u64>().unwrap() + days_num * 24 * 60;
                    let duration = Duration::from_secs(minutes_num * 60);
                    return duration.as_secs().to_string();
                } else {
                    let duration = Duration::from_secs(days_num * 24 * 60 * 60);
                    return duration.as_secs().to_string();
                }
            }
        }
        String::new()
    }

    pub fn convert_to_string(input: String) -> String {
        let secs = input.parse::<u64>().unwrap();
        let days = secs / 86400;
        let minutes = (secs / 60) - days * 24 * 60;
        format!("{}/{}", days, minutes)
    }
}