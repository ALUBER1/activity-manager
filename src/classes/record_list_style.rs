use yew::{Classes, classes};

pub fn record_list_style() -> Classes {
    classes!(
        "flex",
        "self-center",
        "h-[2.5em]",
        "w-9/10",
        "ml-[14px]",
        "border",
        "border-solid",
        "border-(--text-color)",
        "align-middle",
        "[&:last-child]:mb-[15px]",
        "[&:not(:first-child)]:mt-[20px]"
    )
}