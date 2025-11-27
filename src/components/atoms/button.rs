use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub onclick: Callback<bool>,
    pub classes: Classes
}

#[function_component(Button)]
pub fn button(label: &Props) -> Html {
    let onclick = label.onclick.clone();
    let handler = Callback::from(move |_| {
        onclick.emit(true);
    });
    html! {
        <button 
            onclick={handler} 
            type="button"
            classes = {label.classes.clone()}
        >{label.children.clone()}</button>
    }
}
