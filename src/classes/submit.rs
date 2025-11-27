use yew::{Classes, classes};

pub fn submit() -> Classes {
    classes!(
        "transition-[background-color]",
        "duration-300",
        "ease-in-out",
        "[&:hover]:bg-grey",
        "items-center",
        "w-[30px]",
        "rounded-[5px]",
        "ml-[10px]"
    )
}