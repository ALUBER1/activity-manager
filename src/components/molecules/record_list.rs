use yew::{function_component, html, Callback, Html, Properties};

use shared::models::record::Record;

use crate::components::atoms::record_button::RecordButton;

#[derive(Properties, PartialEq, Clone)]
pub struct Props{
    pub list: Vec<Record>,
    pub callback: Callback<Record>
}

#[function_component(RecordList)]
pub fn record_list(records: &Props) -> Html{
    let onclick = records.callback.clone();
    let handler = Callback::from(move |a: Record|{
        onclick.emit(a);
    });
    html!{
        {records.list.clone().into_iter().map(|element|{
            html!{<div class="record-list-style">
                <p>{"name: "}{element.name.clone()}{", date: "}{element.date.clone()}{", time: "}{element.time.clone()}</p>
                <RecordButton id = {element}  onclick = {handler.clone()} ty={"delete"}/>
            </div>}
        }).collect::<Html>()}
    }
}