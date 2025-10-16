use yew::{function_component, html, Callback, Html, Properties};

use crate::components::atoms::record_button::RecordButton;
use shared::models::record::Record;

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
            html!{<div id="record-list">
                <p>{"name: "}{element.name.clone()}{", date: "}{element.date.clone()}{", time: "}{element.time.clone()}</p>
                <RecordButton id = {element}  onclick = {handler.clone()}/>
            </div>}
        }).collect::<Html>()}
    }
}