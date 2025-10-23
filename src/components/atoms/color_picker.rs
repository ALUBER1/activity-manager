use gloo::{console::log, utils::document};
use shared::style::default_colors::DefaultColors;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement};
use yew::prelude::*;

use crate::{app::init_pickr, components::atoms::button::Button};

#[derive(Properties,PartialEq)]
pub struct Props {
    pub call_back: Callback<String>,
    pub item: String,
    pub index: u32
}

#[function_component(ColorPicker)]
pub fn color_picker(prop: &Props) -> Html {

    let callback = prop.call_back.clone();
    let name = use_state(||{prop.item.clone()});
    let index = prop.index.clone();
    let onclick = Callback::from(move |_|{
        let input = document().get_elements_by_class_name("pcr-result");
        let temp = input.get_with_index(index).unwrap().unchecked_into::<HtmlInputElement>();
        callback.emit((*name).clone() + &temp.value() + "#" + &index.to_string());
    });
    
    let callback = prop.call_back.clone();
    let name = use_state(||{prop.item.clone()});
    let index = prop.index.clone();
    let default_onclick = Callback::from(move |_|{
        callback.emit((*name).clone() + &DefaultColors::get(&name) + "#" + &index.to_string());
    });
    
    let name = prop.item.clone();
    use_effect_with((), move |_|{    
        init_pickr("#".to_string()+&name, DefaultColors::get(&name));
    });

    html!{
        <div class="color-picker-container">
            <button id={prop.item.clone()} class="pickr" onfocusout={onclick} style={"background-color: ".to_string() + &DefaultColors::get(&prop.item)} />
            <Button onclick={default_onclick.clone()} id="default" >{"DEFAULT"}</Button>
        </div>
    }
}