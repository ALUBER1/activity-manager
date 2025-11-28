use yew::{function_component, html, Children, Html, Properties};

use crate::classes::settings_classes::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub children: Children,
}

#[function_component(Setting)]
pub fn create_setting(prop: &Props) -> Html {
    html! {
        <>
            <div classes = {setting()}>
                <p classes = {setting_label()}>{prop.label.clone()}</p>
                {prop.children.clone()}
            </div>
            <hr classes = {settings_divisor()}/>
        </>
    }
}
