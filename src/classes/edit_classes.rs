use yew::{Classes, classes};

pub fn edit_form() -> Classes {
    classes!(
        "flex",
        "flex-row",
        "h-full",
        "justify-center",
        "items-center",
        "w-full",
        "[&:first-child]:mt-[0px]"
    )
}

pub fn edit_input() -> Classes {
    classes!(
        "bg-(--input-background-color)",
        "text-(--text-color)",
        "pl-[5px]",
        "mt-[20px]",
        "ml-[10px]",
        "transition-all",
        "delay-1000",
        "border-b",
        "border-b-solid",
        "border-b-white",
        "[&:focus]:outline-none",
        "[&:focus]:border-[#33cfc7]"

    )
}

pub fn editing_button_container() -> Classes {
    classes!(
        "flex",
        "flex-row",
        "w-fit",
        "h-[115px]",
        "justify-between",
        "[&>button]:border",
        "[&>button]:border-solid",
        "[&>button]:border-(--text-color)",
        "[&>button:last-child]:ml-[5px]",
    )
}

pub fn editing_panel() -> Classes {
    classes!(
        "absolute",
        "top-[calc(var(--head-height)_+_10px)]",
        "w-[calc(90%_-_10px)]",
        "h-[calc(98%_-_(var(--head-height)_+_10px))]",
        "bg-(--background-color)",
        "ml-[10px]",
        "z-100",
        "border",
        "border-solid",
        "border-(--text-color)",
        "flex",
        "flex-col",
        "justify-center"
    )
}

pub fn form_fields() -> Classes {
    classes!(
        "flex",
        "flex-col",
        "justify-center",
        "w-[40%]",
        "h-fit",
        "[&>input:first-child]:mt-0"
    )
}

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