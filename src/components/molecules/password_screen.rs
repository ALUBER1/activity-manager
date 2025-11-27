use crate::{
    classes::{password_screen, screen_blocker::screen_blocker, show::show}, components::atoms::{password_input::PasswordInput, password_submit_button::PasswordSubmitButton}, utils::helper::invoke_function_store
};
use gloo::timers::callback::Timeout;
use shared::{models::storage_entry::StorageEntry, style::default_colors::DefaultColors};
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with, use_state, Callback, Html, SubmitEvent};

#[function_component(PasswordScreen)]
pub fn password_screen() -> Html {
    let password = use_state(|| String::new());
    let showing = use_state(|| false);
    let correct = use_state(|| StorageEntry::default());
    let color = use_state(|| DefaultColors::INPUT_BACKGROUND_COLOR);

    let handler = {
        let password_clone = password.clone();
        Callback::from(move |password: String| {
            password_clone.set(password);
        })
    };

    let password_button_onclick = {
        let show_clone = showing.clone();
        Callback::from(move |_| {
            show_clone.set(!*show_clone);
        })
    };

    let submit_handler = {
        let password_clone = password.clone();
        let correct_clone = correct.clone();
        Callback::from(move |event: SubmitEvent| {
            let password_clone = password_clone.clone();
            let correct_clone = correct_clone.clone();
            event.prevent_default();
            spawn_local(async move {
                invoke_function_store(
                    "verify",
                    Some(correct_clone.clone()),
                    Some(StorageEntry::new(
                        "password".to_string(),
                        (*password_clone).clone(),
                    )),
                )
                .await;
            });
        })
    };

    let color_clone = color.clone();
    let correct_clone = correct.clone();
    use_effect_with((*correct).value.clone(), move |correct| {
        if correct.eq("false") {
            color_clone.set(DefaultColors::INVALID_COLOR);
            Timeout::new(700, move || {
                color_clone.set(DefaultColors::INPUT_BACKGROUND_COLOR);
            })
            .forget();
            correct_clone.set(StorageEntry::default());
        }
    });

    html! {
        if !(*correct).value.eq("true") {
            <div classes = {password_screen::password_screen()} >
                <div classes = {screen_blocker()} />
                <form id="password-form" onsubmit={submit_handler}>
                    <PasswordInput callback={handler} show={*showing} color={*color}/>
                    <button classes = {show()} type="button" onclick={password_button_onclick} >
                        <span class="material-symbols-outlined">{
                            if *showing {
                                "visibility_off"
                            } else {
                                "visibility"
                            }
                        }</span>
                    </button>
                    <PasswordSubmitButton>{"login"}</PasswordSubmitButton>
                </form>
            </div>
        }
    }
}
