use yew::prelude::*;

use shared::models::record::Record;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: Record,
    pub onclick: Callback<Record>
}

#[function_component(RecordButton)]
pub fn button(label: &Props) -> Html {
    let onclick = label.onclick.clone();
    let id = label.id.clone();
    let handler = Callback::from(move |_|{
        onclick.emit(id.clone());
    });
    html!{
        <button style = "margin: 0; align-self: center; flex-shrink: 0; width: auto;" onclick = {handler}><span class="material-symbols-outlined">{"delete"}</span> </button>
    }
}