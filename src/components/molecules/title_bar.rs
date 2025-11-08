use yew::prelude::*;

use crate::{components::atoms::title_bar_btn::TitleButton, utils::functions::Functions};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_click: Callback<Functions>
}

#[function_component(TitleBar)]
pub fn button(label: &Props) -> Html {
    let maximize_class = use_state(||{false});
    
    let handler = {
        let maximize_class_clone = maximize_class.clone();
        let onclick = label.on_click.clone();
        Callback::from(move |a: Functions|{
            if a == Functions::Maximize {
                maximize_class_clone.set(!(*maximize_class_clone));
            }
            onclick.emit(a.clone());
        })
    };

    html!{
        <div data-tauri-drag-region="true" id="head">
                <h1 data-tauri-drag-region="true" id="title">{"Activity manager"}</h1>
                <TitleButton id={Functions::Tray} on_click={handler.clone()}><span class="material-symbols-outlined">{"arrow_downward_alt"}</span></TitleButton>
                <TitleButton id={Functions::Minimize} on_click={handler.clone()}><span class="material-symbols-outlined">{"remove"}</span></TitleButton>
                <TitleButton id={Functions::Maximize} on_click={handler.clone()}><span class="material-symbols-outlined">{if !(*maximize_class) {"fullscreen"} else {"fullscreen_exit"}}</span></TitleButton>
                <TitleButton id={Functions::Close} on_click={handler.clone()}><span class="material-symbols-outlined">{"close"}</span></TitleButton>
        </div>
    }
}
