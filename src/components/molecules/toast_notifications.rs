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
                        .enumerate()
                        .map(|(id, element)|{
                            html!{
                                <ToastNotification notification={ToastNotificationModel{ id, ..element }} delete_callback={delete_handler.clone()} />
                            }
                        }
                    ).collect::<Html>()
                }
            </div>
        }
    }
}