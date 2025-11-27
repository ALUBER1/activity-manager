use yew::{Classes, classes};

pub fn edit_form() -> Classes {
    classes!(
        "flex",
        "flex-row",
        "h-full",
        "justify-center",
        "items-center",
        "w-full",
        "[&:first-child]:mt-[0px]"
    )
}