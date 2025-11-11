use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{Callback, Html, NodeRef, Properties, function_component, html, use_effect_with, use_state};

use crate::{models::toast_notification_model::ToastNotificationModel, utils::logger::log};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub notification: ToastNotificationModel,
    pub delete_callback: Callback<ToastNotificationModel>
}

#[function_component(ToastNotification)]
pub fn create_toast_notification(prop: &Props) -> Html {
    let toast: NodeRef = NodeRef::default();
    let visible = use_state(||true);

    use_effect_with(toast.clone(), move |node| {
        if let Some(node) = (*node).get() {
            let tmp = node.unchecked_into::<HtmlElement>();
            Timeout::new(600, move ||{
                tmp.set_class_name("notification-container slide-in");
            }).forget();
        }
    });

    let notification = prop.notification.clone();
    let toast_clone: NodeRef = toast.clone();
    let visible_clone = visible.clone();
    Timeout::new(5600, move||{
        if let Some(node) = toast_clone.get() {
            log("deleting toast: $0", &[&notification.id.to_string()]);
            let tmp = node.unchecked_into::<HtmlElement>();
            tmp.set_class_name("notification-container");
            visible_clone.set(false);
        }
    }).forget();

    let notification = prop.notification.clone();
    let delete_callback = prop.delete_callback.clone();
    use_effect_with(visible.clone(), move |visible|{
        if !**visible {
            log("deleting: $0", &[&notification.id.to_string()]);
            Timeout::new(600, move ||{
                delete_callback.emit(notification);
            }).forget();
        }
    });

    html!{
        <div class="notification-container" ref={toast}>
            <p class="notification-title">{prop.notification.title.clone()}</p>
            <div class="notification-content">
                <p class="notification-message">{prop.notification.message.clone()}</p>
                <div class="notification-progress" />
            </div>
        </div>
    }
}