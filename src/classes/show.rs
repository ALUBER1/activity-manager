use yew::{Classes, classes};

pub fn show() -> Classes {
    classes!(
        "absolute",
        "top-[calc(50%_-_(var(--field-height)_/_2)_-_(var(--head-height)_/_2)_+_3px)]",
        "left-[calc(50%_-_(var(--field-width)_/_2)_+_var(--field-width)_+_10px)]"
    )
}