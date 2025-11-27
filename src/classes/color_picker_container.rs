use yew::{Classes, classes};

pub fn color_picker_container() -> Classes {
    classes!(
        "inline-block",
        "w-100",
        "h-fit",
        "relative",
        "ml-auto",
        "mr-10",
        "outline-none",
        "cursor-pointer",
        "rounded-10"
    )
}