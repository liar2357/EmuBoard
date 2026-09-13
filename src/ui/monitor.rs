use std::sync::Arc;

use gtk::prelude::*;
use gtk::{ApplicationWindow, gdk};
use gtk4_layer_shell::LayerShell;

use crate::event::log::Logger;

pub fn find_monitor_by_name(name: &str, logger: Arc<Logger>) -> Option<gdk::Monitor> {
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

    logger.warn(format!("Failed get monitor info by '{name}'"));
    logger.warn("Fall back to auto selected monitor");

    monitors.item(0)?.downcast::<gdk::Monitor>().ok()
}

pub fn setup_monitor(
    window: &ApplicationWindow,
    monitor_name: &str,
    logger: Arc<Logger>,
) -> Option<(i32, i32)> {
    let monitor = find_monitor_by_name(monitor_name, Arc::clone(&logger))?;

    let width = monitor.geometry().width();
    let height = monitor.geometry().height();

    window.set_monitor(Some(&monitor));

    Some((width, height))
}
