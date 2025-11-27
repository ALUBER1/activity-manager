use yew::{Classes, classes};

pub fn editing_panel() -> Classes {
    classes!(
        "absolute",
        "top-[calc(var(--head-height)_+_10px)]",
        "w-[calc(90%_-_10px)]",
        "h-[calc(98%_-_(var(--head-height)_+_10px))]",
        "bg-(--background-color)",
        "ml-[10px]",
        "z-100",
        "border",
        "border-solid",
        "border-(--text-color)",
        "flex",
        "flex-col",
        "justify-center"
    )
}