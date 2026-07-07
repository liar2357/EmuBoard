use crate::{
    input::structs::{InputCommand, InputSender},
    ui::structs::Keyboard,
};

use evdevil::event::Key;

use std::sync::mpsc::Receiver;

pub fn supported_keys(layout: &Keyboard) -> Vec<Key> {
    use std::collections::HashSet;

    let mut set = HashSet::new();

    for row in &layout.rows {
        for key in &row.keys {
            set.insert(key.key_code());
        }
    }

    set.into_iter().collect()
}

pub fn run_input_thread(rx: Receiver<InputCommand>, keys: Vec<Key>) {
    println!("THREAD START");

    let mut sender = InputSender::new(keys).unwrap();

    while let Ok(cmd) = rx.recv() {
        match cmd {
            InputCommand::KeyDown(key) => {
                println!("DOWN {:?}", key);
                sender.key_down(key).unwrap();
            }

            InputCommand::KeyUp(key) => {
                println!("UP {:?}", key);
                sender.key_up(key).unwrap();
            }

            InputCommand::Shutdown => break,
        }
    }
}
