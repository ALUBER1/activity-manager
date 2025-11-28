use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, Properties, classes, function_component, html};

use crate::classes::select_classes::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub selections: Vec<String>,
    pub onchange: Callback<String>,
    pub selected: String,
}

#[function_component(Select)]
pub fn create_select(prop: &Props) -> Html {
    let onchange = {
        let callback = prop.onchange.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            callback.emit(value);
        })
    };

    html! {
        <div classes = {select_container()}>
            <select classes= { select_comp() } onchange={onchange} >
                {
                    prop.selections.clone().into_iter().map(|element| {
                        html!{
                            <option value={element.clone()} classes = {select_option_comp()} selected={
                                element.eq(&prop.selected)
                            }>{element}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
            <span classes={classes!("material-symbols-outlined", select_arrow())}>{"keyboard_arrow_down"}</span>
        </div>
    }
}
