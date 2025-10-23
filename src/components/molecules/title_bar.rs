use yew::prelude::*;

use crate::{components::atoms::title_bar_btn::TitleButton, functions::Functions};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_click: Callback<Functions>
}

#[function_component(TitleBar)]
pub fn button(label: &Props) -> Html {
    let onclick = label.on_click.clone();

    let maximize_class = use_state(||{false});
    let maximize_class_clone = maximize_class.clone();

    let handler = Callback::from(move |a: Functions|{
        if a == Functions::Maximize {
            maximize_class_clone.set(!(*maximize_class_clone));
        }
        onclick.emit(a.clone());
    });
    html!{
        <div data-tauri-drag-region="true" id="head">
                <h1 data-tauri-drag-region="true" id="title">{"Activity manager"}</h1>
                <TitleButton id={Functions::Minimize} on_click={handler.clone()}><span class="material-symbols-outlined">{"remove"}</span></TitleButton>
                <TitleButton id={Functions::Maximize} on_click={handler.clone()}><span class="material-symbols-outlined">{if !(*maximize_class) {"fullscreen"} else {"fullscreen_exit"}}</span></TitleButton>
                <TitleButton id={Functions::Close} on_click={handler.clone()}><span class="material-symbols-outlined">{"close"}</span></TitleButton>
        </div>
    }
}
