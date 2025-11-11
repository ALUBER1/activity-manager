pub struct DefaultColors;

impl DefaultColors {
    pub const BACKGROUND_COLOR: &str = "#3c3c3c";
    pub const TEXT_COLOR: &str = "#a9a9a9";
    pub const HEAD_BACKGROUND_COLOR: &str = "#272727";
    pub const INPUT_BACKGROUND_COLOR: &str = "#313131";

    pub const INVALID_COLOR: &str = "#cc0b0b";

    pub fn get(input: &str) -> String {
        let toreturn: String = match input {
            "background-color" => Self::BACKGROUND_COLOR.to_string(),
            "text-color" => Self::TEXT_COLOR.to_string(),
            "head-background-color" => Self::HEAD_BACKGROUND_COLOR.to_string(),
            "input-background-color" => Self::INPUT_BACKGROUND_COLOR.to_string(),
            "invalid-color" => Self::INVALID_COLOR.to_string(),
            _ => "".to_string(),
        };

        toreturn
    }
}
