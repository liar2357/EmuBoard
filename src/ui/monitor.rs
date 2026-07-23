use gtk::prelude::*;
use gtk::{ApplicationWindow, gdk};
use gtk4_layer_shell::LayerShell;

fn find_monitor_by_name(name: &str) -> Option<gdk::Monitor> {
    let display = gdk::Display::default()?;
    let monitors = display.monitors();

    if name == "auto" {
        return monitors.item(0)?.downcast::<gdk::Monitor>().ok();
    }

    for i in 0..monitors.n_items() {
        let obj = monitors.item(i)?;

        let monitor = obj.downcast::<gdk::Monitor>().ok()?;

        if monitor.connector().as_deref() == Some(name) {
            return Some(monitor);
        }
    }

    monitors.item(0)?.downcast::<gdk::Monitor>().ok()
}

pub fn setup_monitor(window: &ApplicationWindow, monitor_name: &str) -> Option<i32> {
    let monitor = find_monitor_by_name(monitor_name)?;

    let width = monitor.geometry().width();

    println!(
        "monitor={} width={} height={}",
        monitor_name,
        width,
        monitor.geometry().height()
    );

    window.set_monitor(Some(&monitor));

    Some(width)
}
