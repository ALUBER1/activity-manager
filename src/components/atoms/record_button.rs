use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: i32,
    pub onclick: Callback<i32>
}

#[function_component(RecordButton)]
pub fn button(label: &Props) -> Html {
    let onclick = label.onclick.clone();
    let id = label.id.clone();
    let handler = Callback::from(move |_|{
        onclick.emit(id);
    });
    html!{
        <button onclick = {handler}></button>
    }
}