use yew::{Classes, classes};

pub fn screen_blocker() -> Classes {
    classes!(
        "z-9998",
        "h-full",
        "w-full",
        "bg-(--background-color)"
    )
}