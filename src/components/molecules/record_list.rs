use serde::Deserialize;
use yew::{function_component, html, Callback, Html, Properties};

use crate::components::atoms::record_button::RecordButton;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct Record{
    pub name: String,
    pub date: String,
    pub time: String
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props{
    pub list: Vec<Record>,
    pub callback: Callback<i32>
}

#[function_component(RecordList)]
pub fn record_list(records: &Props) -> Html{
    let onclick = records.callback.clone();
    let handler = Callback::from(move |a: i32|{
        onclick.emit(a);
    });
    html!{
        {records.list.clone().into_iter().enumerate().map(|(index,element)|{
            html!{<>
                <h1>{"name: "}{element.name}{", data: "}{element.date}{", orario: "}{element.time}</h1>
                <RecordButton id = {index as i32}  onclick = {handler.clone()}/>
            </>}
        }).collect::<Html>()}
    }
}