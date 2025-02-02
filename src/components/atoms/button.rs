use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String
}

#[function_component(Button)]
pub fn button(label: &Props) -> Html {
    html!{
        <button>{label.label.clone()}</button>
    }
}