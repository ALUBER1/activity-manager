use yew::{Classes, classes};

pub fn password_screen() -> Classes {
    classes!(
        "absolute",
        "top-(--head-height)",
        "left-[0px]",
        "z-"
    )
}

pub fn password_submit_button() -> Classes {
    classes!(
        "h-(--field-height)",
        "w-(--field-width)",
        "absolute",
        "top-[calc(50%_-_var(--head-height)_/_2)]",
        "left-[calc(50%_-_var(--head-width)_/_2)]",
        "translate-1/2",
        "border",
        "border-solid",
        "border-(--text-color)",
        "mt-[15px]",
        "transition-[background-color]",
        "duration-300",
        "ease-in-out"

    )
}

pub fn screen_blocker() -> Classes {
    classes!(
        "z-9998",
        "h-full",
        "w-full",
        "bg-(--background-color)"
    )
}

pub fn show() -> Classes {
    classes!(
        "absolute",
        "top-[calc(50%_-_(var(--field-height)_/_2)_-_(var(--head-height)_/_2)_+_3px)]",
        "left-[calc(50%_-_(var(--field-width)_/_2)_+_var(--field-width)_+_10px)]"
    )
}