use notify_rust::Notification;

pub fn send_notify(body: &str) {
    let _ = Notification::new().summary("EmuBoard").body(body).show();
}
