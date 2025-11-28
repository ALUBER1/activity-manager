use yew::{Classes, classes};

pub fn notification_container() -> Classes {
    classes!(
        "duration-600",
        "transform-[translate-x-[calc(100%_+_10px)]]",
        "shrink-0",
        "w-[calc(100%_+_10px)]",
        "h-2/10",
        "bg-(--head-background-color)",
        "border",
        "border-solid",
        "border-(--text-color)",
        "mb-[10px]",
        "mr-[10px]",
        "overflow-x-hidden",
        "pointer-events-none"
    )
}

pub fn notifications_container() -> Classes {
    classes!(
        "absolute",
        "right-[0px]",
        "bottom-[0px]",
        "w-4/10",
        "h-[calc(100%_-_var(--head-height))]",
        "flex",
        "flex-col",
        "justify-end",
        "overflow-hidden",
        "z-1000",
        "pointer-events-none"
    )
}

pub fn notification_title() -> Classes {
    classes!(
        "border-b",
        "border-b-solid",
        "border-b-(--text-color)",
        "h-[28px]",
        "text-[1.1em]",
        "truncate",
        "overflow-hidden",
    )
}

pub fn notification_content() -> Classes {
    classes!(
        "w-full",
        "h-[calc(100%_-_28px)]",
        "flex",
        "flex-col",
        "justify-between"
    )
}

pub fn notification_progress() -> Classes {
    classes!(
        "bg-(--invalid-color)",
        "transition-[width]",
        "duration-5000",
        "ease-linear",
        "h-[5px]",
        "w-0/1"
    )
}