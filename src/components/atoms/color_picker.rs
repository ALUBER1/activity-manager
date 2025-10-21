use gloo::{console::log, utils::document};
use shared::style::default_colors::DefaultColors;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast};
use web_sys::{HtmlInputElement};
use yew::prelude::*;

use crate::components::atoms::button::Button;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub call_back: Callback<String>,
    pub item: String
}



#[function_component(ColorPicker)]
pub fn color_picker(prop: &Props) -> Html {

    let callback = prop.call_back.clone();
    let name = use_state(||{prop.item.clone()});
    let onclick = Callback::from(move |_|{
        let input = document().get_elements_by_class_name("pcr-result");
        let temp = input.get_with_index(0).unwrap().unchecked_into::<HtmlInputElement>();
        log!((*name).clone() + &temp.value());
        callback.emit((*name).clone() + &temp.value());
    });

    
    let callback = prop.call_back.clone();
    let name = use_state(||{prop.item.clone()});
    let default_onclick = Callback::from(move |_|{
        callback.emit((*name).clone() + &DefaultColors::get(&name));
    });

    html!{
        <div class="color-picker-container">
            <button id="btn" class="pickr" onfocusout={onclick} />
            <Button onclick={default_onclick.clone()} id="default">{"DEFAULT"}</Button>
        </div>
    }
}