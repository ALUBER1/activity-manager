use gloo::utils::document;
use shared::style::default_colors::DefaultColors;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    app::init_pickr, components::atoms::button::Button, models::setting_value::SettingValue,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub call_back: Callback<SettingValue>,
    pub item: String,
    pub index: u32,
}

#[function_component(ColorPicker)]
pub fn color_picker(prop: &Props) -> Html {
    let onclick = {
        let callback = prop.call_back.clone();
        let name = prop.item.clone();
        let index = prop.index.clone();
        Callback::from(move |_| {
            let input = document().get_elements_by_class_name("pcr-result");
            let temp = input
                .get_with_index(index)
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            callback.emit(SettingValue::new(
                name.clone(),
                temp.value(),
                true,
                index.to_string(),
            ));
        })
    };

    let default_onclick = {
        let callback = prop.call_back.clone();
        let index = prop.index.clone();
        let name = prop.item.clone();
        Callback::from(move |_| {
            callback.emit(SettingValue::new(
                name.clone(),
                DefaultColors::get(&name),
                true,
                index.to_string(),
            ));
        })
    };

    let name = prop.item.clone();
    use_effect_with((), move |_| {
        init_pickr(format!("#{}", name), DefaultColors::get(&name));
    });

    html! {
        <div class="color-picker-container">
            <button id={prop.item.clone()} class="pickr" onfocusout={onclick} style={"background-color: ".to_string() + &DefaultColors::get(&prop.item)} />
            <Button onclick={default_onclick.clone()} id="default" >{"DEFAULT"}</Button>
        </div>
    }
}
