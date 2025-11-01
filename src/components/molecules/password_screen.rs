use yew::{Callback, Html, Properties, SubmitEvent, function_component, html, use_state};
use crate::components::atoms::{password_input::PasswordInput, submit_button::SubmitButton};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub callback: Callback<String>
}

#[function_component(PasswordScreen)]
pub fn password_screen(prop: &Prop) -> Html {
    let password = use_state(||String::new());
    let password_clone = password.clone();
    let handler = Callback::from(move |password: String| {
        password_clone.set(password);
    });

    let show = use_state(||false);
    let show_clone = show.clone();
    let password_button_onclick = Callback::from(move |_| {
        show_clone.set(!*show_clone);
    });

    let logged = use_state(||false);
    let logged_clone = logged.clone();

    let submit_handler = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        logged_clone.set(true);
    });
    
    html!{
        if !*logged {
            <div id="password-screen" >
                <div id="screen-blocker" />
                <form id="password-form" onsubmit={submit_handler}>
                    <PasswordInput callback={handler} show={*show}/>
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