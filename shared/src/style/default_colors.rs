pub struct DefaultColors;

impl DefaultColors {
    pub const BACKGROUND_COLOR: &str = "#3c3c3c";
    pub const TEXT_COLOR: &str = "#a9a9a9";
    pub const HEAD_COLOR: &str = "#272727";
    pub const INPUT_COLOR: &str = "#313131";

    pub fn get(input: &str) -> String {
        let toreturn: String = match input {
            "background-color" => Self::BACKGROUND_COLOR.to_string(),
            "text-color" => Self::TEXT_COLOR.to_string(),
            "head-color" => Self::HEAD_COLOR.to_string(),
            "input-color" => Self::INPUT_COLOR.to_string(),
            _ => "".to_string(),
        };

        toreturn
    }
}