use yew::{Classes, classes};

pub fn cancel() -> Classes {
    classes!(
        "transition-(background-color)",
        "duration-300",
        "ease-in-out",
        "items-center",
        "w-[30px]",
        "rounded-[5px]",
        "ml-[10px]",
        "[&:hover]:bg-grey"
    )
}