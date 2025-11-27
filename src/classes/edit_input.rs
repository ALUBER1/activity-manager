use yew::{Classes, classes};

pub fn edit_input() -> Classes {
    classes!(
        "bg-(--input-background-color)",
        "text-(--text-color)",
        "pl-[5px]",
        "mt-[20px]",
        "ml-[10px]",
        "transition-all",
        "delay-1000",
        "border-b",
        "border-b-solid",
        "border-b-white",
        "[&:focus]:outline-none",
        "[&:focus]:border-[#33cfc7]"

    )
}