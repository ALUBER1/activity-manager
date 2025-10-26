use std::time::Duration;
use web_sys::Element;
use yew::{function_component, html, use_state, Callback, Html, NodeRef, Properties};

use crate::components::atoms::{button::Button, color_picker::ColorPicker, notification_input::NotificationInput, setting::Setting};

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

    let on_change = prop.callback.clone();
    let notification_delay_handle = Callback::from(move |input: String|{
        if !input.chars().any(|c|{c.is_alphabetic()}) {
            let mut split = input.split("/");
            if let Some(days) = split.nth(0) {
                let days_num: u64 = days.parse::<u64>().unwrap();
                if let Some(minutes) = split.nth(0) {
                    let minutes_num: u64 = minutes.parse::<u64>().unwrap() + days_num * 24 * 60;
                    let duration = Duration::from_secs(minutes_num * 60);
                    on_change.emit(duration.as_secs().to_string());
                } else {
                    let duration = Duration::from_secs(days_num * 24 * 60 * 60);
                    on_change.emit(duration.as_secs().to_string());
                }
            }
            
        }
    });

    html!{
        <div id="settings-container">
            <div id="settings-panel" class="hide-panel" ref={test.clone()}>
                <Setting label={"background color"}><ColorPicker item="background-color" call_back={get_input_values.clone()} index=0 /></Setting>
                <Setting label={"header color"}><ColorPicker item="head-background-color" call_back={get_input_values.clone()} index=1 /></Setting>
                <Setting label={"input color"}><ColorPicker item="input-background-color" call_back={get_input_values.clone()} index=2 /></Setting>
                <Setting label={"text color"}><ColorPicker item="text-color" call_back={get_input_values.clone()} index=3 /></Setting>
                <Setting label={"notification delay"}><NotificationInput name="days/minutes" on_change={notification_delay_handle.clone()} color={" "} /></Setting>
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", *class)}>{"settings"}</span></Button>
        </div>
    }
}