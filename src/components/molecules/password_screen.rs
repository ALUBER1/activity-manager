use yew::{Callback, Html, Properties, function_component, html, use_state};
use crate::components::atoms::{password_input::PasswordInput, submit_button::SubmitButton};

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub callback: Callback<String>
}

#[function_component(PasswordScreen)]
pub fn password_screen(prop: &Prop) -> Html {
    let callback = prop.callback.clone();
    let handler = Callback::from(move |password: String| {
        callback.emit(password);
    });

    let show = use_state(||false);
    let show_clone = show.clone();
    let password_button_onclick = Callback::from(move |_| {
        show_clone.set(!*show_clone);
    });
    html!{
        <div id="password-screen">
            <div id="screen-blocker" />
            <form id="password-form">
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