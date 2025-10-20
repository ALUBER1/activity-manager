use gloo::console::log;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast};
use web_sys::{HtmlInputElement};
use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub call_back: Callback<String>,
    pub value: String
}



#[function_component(ColorPicker)]
pub fn color_picker(prop: &Props) -> Html {

    let callback = prop.call_back.clone();
    let on_change = Callback::from(move |event: InputEvent| {
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        
        log!(input.clone());
        callback.emit(input);
    });

    html!{
        <div class="color-picker-container">
            <input id="btn" class="pickr" />
        </div>
    }
}