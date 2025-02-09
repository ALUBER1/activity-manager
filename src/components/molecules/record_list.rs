use serde::Deserialize;
use yew::{function_component, html, Callback, Html, Properties};

use crate::components::atoms::record_button::RecordButton;

#[derive(Properties, PartialEq, Clone, Deserialize, Default)]
pub struct Record{
    pub name: String,
    pub date: String,
    pub time: String
}

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
            html!{<div style = "display: flex; align: center;">
                <h1 style = "margin: 15px 20px 15px 20px; align-self: center;">{"name: "}{element.name.clone()}{", data: "}{element.date.clone()}{", orario: "}{element.time.clone()}</h1>
                <RecordButton id = {element}  onclick = {handler.clone()}/>
            </div>}
        }).collect::<Html>()}
    }
}