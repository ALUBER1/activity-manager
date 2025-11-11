use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub id: String,
    pub onclick: Callback<bool>,
}

#[function_component(Button)]
pub fn button(label: &Props) -> Html {
    let onclick = label.onclick.clone();
    let handler = Callback::from(move |_| {
        onclick.emit(true);
    });
    html! {
        <button id={label.id.clone()} onclick={handler} type="button">{label.children.clone()}</button>
    }
}
