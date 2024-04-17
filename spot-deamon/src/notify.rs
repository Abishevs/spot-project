use notify_rust::Notification;

pub trait Notifier {
    fn send(&self, title: &str, message: &str);
}


pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn send(&self, title: &str, message: &str) {
        Notification::new()
            .summary(title)
            .body(&message)
            .show().expect("Notfication send failed");

    }
    
}
