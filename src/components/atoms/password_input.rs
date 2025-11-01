use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub callback: Callback<String>,
    pub show: bool
}

#[function_component(PasswordInput)]
pub fn password_input(prop: &Prop) -> Html {
    let callback = prop.callback.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        callback.emit(value);
    });
    html!{
        <input id="password-input" onchange={onchange} type={
            if prop.show {
                "text"
            } else {
                "password"
            }
        } />
    }
}