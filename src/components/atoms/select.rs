use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selections: Vec<String>,
    pub onchange: Callback<String>,
    pub selected: String
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
        <div class="select-container">
            <select class="select-comp" onchange={onchange} >
                {
                    prop.selections.clone().into_iter().map(|element| {
                        html!{
                            <option value={element.clone()} class="select-option-comp" selected={
                                element.eq(&prop.selected)
                            }>{element}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
            <span class={"material-symbols-outlined select-arrow"}>{"keyboard_arrow_down"}</span>
        </div>
    }
}