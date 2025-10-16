use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub id: String
}

#[function_component(Button)]
pub fn button(label: &Props) -> Html {
    html!{
        <button id={label.id.clone()}>{label.children.clone()}</button>
    }
}