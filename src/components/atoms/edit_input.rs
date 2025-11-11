use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub on_change: Callback<String>,
    pub color: String,
    pub value: String
}

#[function_component(EditInput)]
pub fn text_input(props: &Props) -> Html{
    let onchange = {
        let on_changecall = props.on_change.clone();
        Callback::from(move |event: Event|{
            let input = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            on_changecall.emit(input);
        })
    };
    
    html!{
        <input type="text" autocomplete = "off" name = {props.name.clone()} placeholder = {props.name.clone()} onchange = {onchange} style = {format!("background-color: {}", props.color.clone())} value={props.value.clone()} />
    }
}
