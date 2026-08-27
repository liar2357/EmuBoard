use crate::{config::loader::try_get_config_path, event::structs::ReloadEvent};
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

    eprintln!("Thread end: Change");
    Ok(())
}
