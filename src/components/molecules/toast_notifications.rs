use chrono::{Duration, Local};
use yew::{Callback, Html, Properties, function_component, html};

use crate::{components::atoms::toast_notification::ToastNotification, models::toast_notification_model::ToastNotificationModel};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub notifications: Vec<ToastNotificationModel>,
    pub delete_callback: Callback<ToastNotificationModel>
}

#[function_component(ToastNotifications)]
pub fn create_toast_notifications(prop: &Props) -> Html {
    let delete_handler = {
        let delete_callback = prop.delete_callback.clone();
        Callback::from(move |notification: ToastNotificationModel| {
            delete_callback.emit(notification);
        })
    };

    html!{
        if prop.notifications.len() != 0 {
            <div class="notifications-container" >
                {
                    prop.notifications.clone()
                        .into_iter()
                        .rev()
                        .map(|element|{
                            if (Duration::milliseconds(5600) - (Local::now().time() - element.created_at)).num_milliseconds().max(0) > 10 {
                                html!{
                                    <ToastNotification key={element.id} notification={element.clone()} delete_callback={delete_handler.clone()} />
                                }
                            } else {
                                html!{}
                            }
                        }
                    ).collect::<Html>()
                }
            </div>
        }
    }
}
