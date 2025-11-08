use yew::prelude::*;

use crate::utils::functions::Functions;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub id: Functions,
    pub on_click: Callback<Functions>
}

#[function_component(TitleButton)]
pub fn button(label: &Props) -> Html {
    let handler = {
        let onclick = label.on_click.clone();
        let id = label.id.clone();
        Callback::from(move |_|{
            onclick.emit(id.clone());
        })
    };
    html!{
        <button id={label.id.to_string()} onclick={handler}>{for label.children.iter()}</button>
    }
}