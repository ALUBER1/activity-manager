use yew::{Classes, classes};

pub fn form_fields() -> Classes {
    classes!(
        "flex",
        "flex-col",
        "justify-center",
        "w-[40%]",
        "h-fit",
        "[&>input:first-child]:mt-0"
    )
}