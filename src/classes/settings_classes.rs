use yew::{Classes, classes};

pub fn settings_container() -> Classes {
    classes!(
        "z-1000",
        "w-fit",
        "absolute",
        "top-[52.5px]",
        "h-[calc(100%_-_52.5px)]",
    )
}

pub fn setting() -> Classes {
    classes!(
        "transition-opacity",
        "duration-450",
        "ease-in-out",
        "@[(width:300px)]:opacity-100",
        "flex",
        "w-full",
        "items-center",
        "relative",
        "flex-row",
        "opacity-0",
        "justify-between",
        "h-[26px]"
    )
}

pub fn setting_label() -> Classes {
    classes!(
        "ml-[15px]",
        "w-[10em]",
        "text-[15px]",
        "text-left",
        "whitespace-nowrap",
        "wrap-normal",
        "text-nowrap",
        "select-none",
        "border-r",
        "border-r-solid",
        "border-r-(--background-color)",
        "h-full",
        "[&+input]:ml-[10px]"
    )
}

pub fn settings_divisor() -> Classes {
    classes!(
        "relative",
        "w-full",
        "h-[1px]",
        "border-0",
        "border-t",
        "border-t-solid",
        "border-t-(--background-color)",
        "p-0",
        "opacity-0",
        "transition-opacity",
        "duration-450",
        "ease-in-out",
        "@[(width:300px)]:opacity-100",
    )
}

pub fn settings() -> Classes {
    classes!(
        "bg-(--head-background-color)",
        "fixed",
        "w-[40px]",
        "h-[40px]",
        "rounded-[20px]",
        "bottom-[0px]",
        "mb-[10px]",
        "ml-[10px]",
        "inline"
    )
}