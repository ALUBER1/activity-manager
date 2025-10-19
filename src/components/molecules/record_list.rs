use yew::{function_component, html, Callback, Html, Properties};

use shared::models::record::Record;

use crate::components::atoms::record_button::RecordButton;

#[derive(Properties, PartialEq, Clone)]
pub struct Props{
    pub list: Vec<Record>,
    pub delete_callback: Callback<Record>,
    pub edit_callback: Callback<Record>
}

#[function_component(RecordList)]
pub fn record_list(records: &Props) -> Html{
    let onclick = records.delete_callback.clone();
    let delete_handler = Callback::from(move |a: Record|{
        onclick.emit(a);
    });

    let onclick = records.delete_callback.clone();
    let edit_handler = Callback::from(move |a: Record|{
        onclick.emit(a)
    });
    html!{
        {records.list.clone().into_iter().map(|element|{
            html!{<div class="record-list-style">
                <p>{"name: "}{element.name.clone()}{", date: "}{element.date.clone()}{", time: "}{element.time.clone()}</p>
                <RecordButton id = {element.clone()}  onclick = {delete_handler.clone()} ty={"delete"}/>
                <RecordButton id = {element}  onclick = {edit_handler.clone()} ty={"edit"}/>
            </div>}
        }).collect::<Html>()}
    }
}