use yew::{Classes, classes};

pub fn editing_button_container() -> Classes {
    classes!(
        "flex",
        "flex-row",
        "w-fit",
        "h-[115px]",
        "justify-between",
        "[&>button]:border",
        "[&>button]:border-solid",
        "[&>button]:border-(--text-color)",
        "[&>button:last-child]:ml-[5px]",
    )
}