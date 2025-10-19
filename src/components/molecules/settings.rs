use gloo::{timers::callback::Timeout};
use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{atoms::button::Button};


#[function_component(Settings)]
pub fn create_setting() -> Html {
    let class = use_state(||String::new());
    let cloned_class = class.clone();
    let listener = Callback::from(move |_| {
        cloned_class.set(String::from("spin"));
        let clone = cloned_class.clone();
        Timeout::new(700, move || {
            clone.set(String::new());
        }).forget();
    });

    html!{
        <div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", *class)}>{"settings"}</span></Button>
        </div>
    }
}