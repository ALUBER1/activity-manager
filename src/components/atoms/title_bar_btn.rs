use yew::prelude::*;

use crate::functions::Functions;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub id: Functions,
    pub on_click: Callback<Functions>
}

#[function_component(TitleButton)]
pub fn button(label: &Props) -> Html {
    let onclick = label.on_click.clone();
    let id = label.id.clone();
    let handler = Callback::from(move |_|{
        onclick.emit(id.clone());
    });
    html!{
        <button id={label.id.to_string()} onclick={handler}>{label.label.clone()}</button>
    }
}