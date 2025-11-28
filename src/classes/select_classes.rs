use yew::{Classes, classes};

pub fn select_container() -> Classes {
    classes!(
        "flex",
        "w-44/100",
        "h-full",
        "align-middle",
        "justify-center"
    )
}

pub fn select_comp() -> Classes {
    classes!(
        "w-[36px]",
        "h-full",
        "text-left",
        "pl-[5px]",
        "appearance-none",
        "bg-transparent",
        "text-(--text-color)",
        "z-101"
    )
}

pub fn select_arrow() -> Classes {
    classes!(
        "transform-[rotate-180]",
        "absolute",
        "left-78/100",
        "transition-[transform]",
        "duration-350",
        "ease-in-out",
        "z-100"
    )
}

pub fn select_option_comp() -> Classes {
    classes!(
        "bg-(--head-background-color)",
        "text-(--text-color)"
    )
}