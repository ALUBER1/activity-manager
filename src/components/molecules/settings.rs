use web_sys::Element;
use yew::{function_component, html, use_state, Callback, Html, NodeRef};

use crate::components::atoms::{button::Button, setting::Setting};


#[function_component(Settings)]
pub fn create_setting() -> Html {
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


    html!{
        <div id="settings-container">
            <div id="settings-panel" class="hide-panel" ref={test.clone()}>
                <Setting label={"test"}><input type="range"/></Setting>
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", *class)}>{"settings"}</span></Button>
        </div>
    }
}