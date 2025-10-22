use gloo::{console::log, timers::callback::Timeout};
use web_sys::Element;
use yew::{function_component, html, use_state, Callback, Html, NodeRef, Properties};

use crate::components::atoms::{button::Button, color_picker::ColorPicker, setting::Setting};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<String>
}

#[function_component(Settings)]
pub fn create_setting(prop: &Props) -> Html {
    let class = use_state(||String::new());
    let cloned_class: yew::UseStateHandle<String> = class.clone();

    
    let test: NodeRef = NodeRef::default();
    let cloned_test = test.clone();
    let listener = Callback::from(move |_| {
        
        let element = cloned_test.cast::<Element>().unwrap();

        if element.class_name().eq("show-panel") {    
            cloned_class.set(String::new());
            element.set_class_name("hide-panel");
        } else {
            cloned_class.set(String::from("spin"));
            element.set_class_name("show-panel");
        }
    });

    let on_change = prop.callback.clone();
    let get_input_values = Callback::from(move |input: String|{
        on_change.emit(input);
    });

    html!{
        <div id="settings-container">
            <div id="settings-panel" class="hide-panel" ref={test.clone()}>
                <Setting label={"background color"}><ColorPicker item="background-color" call_back={get_input_values.clone()} index=0 /></Setting>
                <Setting label={"header color"}><ColorPicker item="head-background-color" call_back={get_input_values.clone()} index=1 /></Setting>
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", *class)}>{"settings"}</span></Button>
        </div>
    }
}