use gloo::{console::log, timers::callback::Timeout};
use shared::style::default_colors::DefaultColors;
use yew::{Callback, Html, Properties, SubmitEvent, function_component, html, use_state};
use crate::{components::atoms::{password_input::PasswordInput, submit_button::SubmitButton}, utils::logger::log};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub callback: Callback<String>,
    pub invalid: bool
}

#[function_component(PasswordScreen)]
pub fn password_screen(prop: &Prop) -> Html {
    let password = use_state(||String::new());
    let show = use_state(||false);
    let color = use_state(||String::new());
    
    let timer = 1000;
    
    let password_clone = password.clone();
    let handler = Callback::from(move |password: String| {
        password_clone.set(password);
    });
    
    let show_clone = show.clone();
    let password_button_onclick = Callback::from(move |_| {
        show_clone.set(!*show_clone);
    });
    
    
    let callback = prop.callback.clone();
    let color_clone = color.clone();
    let invalid = prop.invalid.clone();
    let password_clone = password.clone();
    let submit_handler = Callback::from(move |event: SubmitEvent| {
        callback.emit((*password_clone).clone());
        event.prevent_default();
        if !invalid {
            
        } else {
            let color_clone = color_clone.clone();
            color_clone.set(DefaultColors::INVALID_COLOR.to_string());
            Timeout::new(timer, move || {
                color_clone.set("".to_string());
            }).forget();
        }
    });

    html!{
        <div id="password-screen" >
            <div id="screen-blocker" />
            <form id="password-form" onsubmit={submit_handler}>
                <PasswordInput callback={handler} show={*show} color={(*color).clone()}/>
                <button id="show" type="button" onclick={password_button_onclick} >
                    <span class="material-symbols-outlined">{
                        if *show {
                            "visibility_off"
                        } else {
                            "visibility"
                        }
                    }</span>
                </button>
                <SubmitButton id="password-submit-button">{"login"}</SubmitButton>
            </form>
        </div>
    }
    
}