use chrono::{Duration, Local};
use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, NodeRef, Properties,
};

use crate::{classes::notification_classes::{notification_container, notification_content, notification_progress, notification_title}, models::toast_notification_model::ToastNotificationModel};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub notification: ToastNotificationModel,
    pub delete_callback: Callback<ToastNotificationModel>,
}

#[function_component(ToastNotification)]
pub fn create_toast_notification(prop: &Props) -> Html {
    let toast: NodeRef = NodeRef::default();
    let visible = use_state(|| true);

    use_effect_with(toast.clone(), move |node| {
        if let Some(node) = (*node).get() {
            let tmp = node.unchecked_into::<HtmlElement>();
            Timeout::new(600, move || {
                tmp.set_class_name("notification-container slide-in");
            })
            .forget();
        }
    });

    let toast_clone: NodeRef = toast.clone();
    let visible_clone = visible.clone();
    let timeout =
        Duration::milliseconds(5600) - (Local::now().time() - prop.notification.created_at);

    Timeout::new(timeout.num_milliseconds().max(0) as u32, move || {
        if let Some(node) = toast_clone.get() {
            let tmp = node.unchecked_into::<HtmlElement>();
            tmp.set_class_name("notification-container");
            visible_clone.set(false);
        }
    })
    .forget();

    let notification = prop.notification.clone();
    let delete_callback = prop.delete_callback.clone();
    use_effect_with(visible.clone(), move |visible| {
        if !**visible {
            Timeout::new(600, move || {
                delete_callback.emit(notification);
            })
            .forget();
        }
    });

    html! {
        <div classes = {notification_container()} ref={toast}>
            <p classes = {notification_title()}>{prop.notification.title.clone()}</p>
            <div classes = {notification_content()}>
                <p>{prop.notification.message.clone()}</p>
                <div classes = {notification_progress()} />
            </div>
        </div>
    }
}
