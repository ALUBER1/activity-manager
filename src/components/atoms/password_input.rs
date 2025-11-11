use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub callback: Callback<String>,
    pub show: bool,
    pub color: String,
}

#[function_component(PasswordInput)]
pub fn password_input(prop: &Prop) -> Html {
    let onchange = {
        let callback = prop.callback.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            callback.emit(value);
        })
    };
    html! {
        <input id="password-input" onchange={onchange} type={
            if prop.show {
                "text"
            } else {
                "password"
            }
        } style={format!("background-color: {}", prop.color)}/>
    }
}
