use gloo::timers::callback::Timeout;
use shared::style::default_colors::DefaultColors;
use yew::{Callback, Html, Properties, SubmitEvent, function_component, html, use_state};
use crate::components::atoms::{password_input::PasswordInput, submit_button::SubmitButton};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub correct_password: String
}

#[function_component(PasswordScreen)]
pub fn password_screen(prop: &Prop) -> Html {
    let logged = use_state(||false);
    let password = use_state(||String::new());
    let show = use_state(||false);
    let color = use_state(||String::new());
    
    let timer = 3000;

    let password_clone = password.clone();
    let handler = Callback::from(move |password: String| {
        password_clone.set(password);
    });

    let show_clone = show.clone();
    let password_button_onclick = Callback::from(move |_| {
        show_clone.set(!*show_clone);
    });

    
    let logged_clone = logged.clone();
    let password_clone = password.clone();
    let correct_password = prop.correct_password.clone();
    let color_clone = color.clone();
    let submit_handler = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        if correct_password.eq(&*password_clone) {
            logged_clone.set(true);
        } else {
            let color_clone = color_clone.clone();
            color_clone.set(DefaultColors::INVALID_COLOR.to_string());
            Timeout::new(timer, move || {
                color_clone.set("".to_string());
            }).forget();
        }
    });

    html!{
        if !*logged {
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
}