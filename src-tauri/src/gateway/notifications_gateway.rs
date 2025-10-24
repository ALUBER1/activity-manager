use shared::models::notification::Notification;
use tauri::AppHandle;
use tauri_plugin_notifications::NotificationsExt;

pub struct NotificationGateway;

impl NotificationGateway {
    pub fn send_notification(app: &AppHandle, notification: Notification) -> Result<(), tauri_plugin_notifications::Error> {
        app.notifications()
            .builder()
            .body(notification.body)
            .title(notification.title)
            .show()
    }
}