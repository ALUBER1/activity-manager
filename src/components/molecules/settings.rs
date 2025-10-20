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
    let cloned_class = class.clone();

    
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

    let mut color_pick_value = use_state(||{String::from("#3c3c3c")});

    let color_pick_clone = color_pick_value.clone();
    let on_change = prop.callback.clone();
    let get_input_values = Callback::from(move |input: String|{
        color_pick_clone.set(input.clone());
        on_change.emit(input);
    });

    let on_change = prop.callback.clone();
    let color_pick_clone = color_pick_value.clone();
    let default_onclick = Callback::from(move |_|{
        color_pick_clone.set("#3c3c3c".to_string());
        on_change.emit("default".to_string());
    });


    html!{
        <div id="settings-container">
            <div id="settings-panel" class="hide-panel" ref={test.clone()}>
                <hr class="settings-divisor"/>
                <Setting label={"background color"}><ColorPicker value={(*color_pick_value).clone()} call_back={get_input_values.clone()}/><Button onclick={default_onclick.clone()} id="default">{"default"}</Button></Setting>
                <hr class="settings-divisor"/>
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", *class)}>{"settings"}</span></Button>
        </div>
    }
}