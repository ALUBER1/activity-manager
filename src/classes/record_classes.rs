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

pub fn record_list() -> Classes {
    classes!(
        "h-full",
        "w-full",
        "overflow-x-hidden",
        "overflow-y-auto",
        "[&::-webkit-scrollbar]:w-[20px]",
        "[&::-webkit-scrollbar-thumb]:bg-(--text-color)",
        "[&::-webkit-scrollbar-thumb]:rounded-[10px]",
        "[&::-webkit-scrollbar-thumb]:border",
        "[&::-webkit-scrollbar-thumb]:border-solid",
        "[&::-webkit-scrollbar-thumb]:border-(--background-color)",
        "[&::-webkit-scrollbar-track]:bg-transparent",
    )
}

pub fn record_button() -> Classes {
    classes!(
        "relative",
        "h-[26px]",
        "align-middle",
        "top-1/2",
        "translate-y-1/2",
        ""
    )
}