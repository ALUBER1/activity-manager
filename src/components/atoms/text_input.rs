use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub on_change: Callback<String>,
    pub color: String
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html{
    let on_changecall = props.on_change.clone();
    let onchange = Callback::from(move |event: Event|{
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        on_changecall.emit(input);
    });
    html!{
        <input type="text" autocomplete = "off" name = {props.name.clone()} placeholder = {props.name.clone()} onchange = {onchange} style = {format!("background-color: {}", props.color.clone())}/>
    }
}
