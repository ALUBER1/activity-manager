use yew::{Html, Properties, function_component, html, Callback};
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
    html!{
        <div id="password-screen">
            <div id="screen-blocker" />
            <form id="password-form">
                <PasswordInput callback={handler}/>
                <SubmitButton id="password-submit-button">{"login"}</SubmitButton>
            </form>
        </div>
    }
}