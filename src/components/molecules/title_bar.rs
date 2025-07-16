use yew::prelude::*;

use crate::{components::atoms::title_bar_btn::TitleButton, functions::Functions};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_click: Callback<Functions>
}

#[function_component(TitleBar)]
pub fn button(label: &Props) -> Html {
    let onclick = label.on_click.clone();
    let handler = Callback::from(move |a: Functions|{
        onclick.emit(a.clone());
    });
    html!{
        <div data-tauri-drag-region="true" id="head">
                <h1 data-tauri-drag-region="true" id="title">{"Activity manager"}</h1>
                <TitleButton label={"-"} id={Functions::minimize} on_click={handler.clone()}></TitleButton>
                <TitleButton label={"[]"} id={Functions::maximize} on_click={handler.clone()}></TitleButton>
                <TitleButton label={"X"} id={Functions::close} on_click={handler.clone()}></TitleButton>
        </div>
    }
}