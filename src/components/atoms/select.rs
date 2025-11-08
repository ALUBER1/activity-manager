use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selections: Vec<String>,
    pub onchange: Callback<String>
}

#[function_component(Select)]
pub fn create_select(prop: &Props) -> Html {
    let onchange = {
        let callback = prop.onchange.clone();
        Callback::from(move |e: Event| {
            let value = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            callback.emit(value);
        })
    };

    html! {
        <select class="select-comp" onchange={onchange}>
            {
                prop.selections.clone().into_iter().map(|element| {
                    html!{
                        <option value={element.clone()}>{element}</option>
                    }
                }).collect::<Html>()
            }
        </select>
    }
}