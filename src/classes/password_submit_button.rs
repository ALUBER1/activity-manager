use yew::{Classes, classes};

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