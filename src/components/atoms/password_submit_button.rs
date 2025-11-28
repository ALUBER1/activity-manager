use yew::prelude::*;

use crate::classes::password_classes::password_submit_button;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(PasswordSubmitButton)]
pub fn button(label: &Props) -> Html {
    html! {
        <button 
            classes = {password_submit_button()}
        >{label.children.clone()}</button>
    }
}
