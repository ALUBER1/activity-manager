use yew::prelude::*;

use shared::models::record::Record;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: Record,
    pub onclick: Callback<Record>,
    pub ty: String,
}

#[function_component(RecordButton)]
pub fn button(label: &Props) -> Html {
    let handler = {
        let onclick = label.onclick.clone();
        let id = label.id.clone();
        Callback::from(move |_| {
            onclick.emit(id.clone());
        })
    };
    html! {
        <button onclick = {handler}><span class="material-symbols-outlined">{label.ty.clone()}</span> </button>
    }
}
