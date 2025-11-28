use yew::{Classes, classes};

pub fn title() -> Classes {
    classes!(
        "grow",
        "text-[15px]",
        "text-(--text-color)",
        "my-[15px]",
        "mx-[20px]",
        "self-center",
        "select-none",
        "[&:hover]:cursor-default"
    )
}