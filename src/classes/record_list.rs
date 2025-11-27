use yew::{Classes, classes};

pub fn record_list() -> Classes {
    classes!(
        "h-full",
        "w-full",
        "overflow-x-hidden",
        "overflow-y-auto",
        "[&::-webkit-scrollbar]:w-[20px]",
        "[&::-webkit-scrollbar-thumb]:bg-(--text-color)",
        "[&::-webkit-scrollbar-thumb]:rounded-[10px]",
        "[&::-webkit-scrollbar-thumb]:border",
        "[&::-webkit-scrollbar-thumb]:border-solid",
        "[&::-webkit-scrollbar-thumb]:border-(--background-color)",
        "[&::-webkit-scrollbar-track]:bg-transparent",
    )
}