use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children
}

#[function_component(Button)]
pub fn button(label: &Props) -> Html {
    html!{
        <button>{label.children.clone()}</button>
    }
}