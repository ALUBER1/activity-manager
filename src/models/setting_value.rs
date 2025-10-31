#[derive(Debug)]
pub struct SettingValue {
    pub setting: String,
    pub value: String,
    pub index: String,
    pub from_color_picker: bool
}

impl SettingValue {
    pub fn new(setting: String, value: String, picker: bool, index: String) -> Self {
        SettingValue { setting, value, from_color_picker: picker, index }
    }

    pub fn deserialize(input: String) -> Self {
        let mut splits = input.split("#");

        SettingValue { setting: splits.nth(0).unwrap().to_string(), value: splits.nth(0).unwrap().to_string(), from_color_picker: true, index: splits.nth(0).unwrap().to_string() }
    }

    pub fn serialize(&self) -> String {
        format!("{}#{}#{}", self.setting, self.value, self.index)
    }
}