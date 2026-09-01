use crate::{
    config::loader::try_get_config_path,
    event::{notify::send_notify, structs::ReloadEvent},
    ui::monitor::find_monitor_by_name,
};
use gtk::{glib::ControlFlow, prelude::*};
use notify::{Config as NotifyConfig, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{Receiver, Sender};

pub fn watch_file_change(tx: Sender<ReloadEvent>, rx: Receiver<()>) -> anyhow::Result<()> {
    eprintln!("Thread Start: Change");

    let config_path = try_get_config_path()?;
    let config_dir = config_path
        .parent()
        .expect("config path has no parent")
        .to_path_buf();

    let mut watcher = RecommendedWatcher::new(
        move |result: notify::Result<notify::Event>| {
            let Ok(event) = result else {
                return;
            };

            let is_changed = event.paths.iter().any(|path| path == &config_path);

            if !is_changed {
                return;
            }

            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    let _ = tx.send(ReloadEvent::ChangeConfigFile);
                }
                _ => {}
            }
        },
        NotifyConfig::default(),
    )?;

    watcher.watch(&config_dir, RecursiveMode::NonRecursive)?;

    let _ = rx.recv();

    eprintln!("Thread End: Change");
    Ok(())
}

pub fn monitor_width(monitor_name: &str) -> Option<i32> {
    let monitor = find_monitor_by_name(monitor_name)?;
    Some(monitor.geometry().width())
}

pub fn watch_monitor_change(
    monitor_name: &str,
    tx: &Sender<ReloadEvent>,
    previous_width: &mut Option<i32>,
) -> ControlFlow {
    let current_width = monitor_width(monitor_name);

    if current_width != *previous_width {
        let old_width = previous_width.unwrap_or(0);
        let new_width = current_width.unwrap_or(0);

        send_notify(format!("monitor width changed: {} -> {}", old_width, new_width).as_str());

        *previous_width = current_width;

        let _ = tx.send(ReloadEvent::ChangeMonitorWidth);
    }

    ControlFlow::Continue
}
