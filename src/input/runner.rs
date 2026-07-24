use crate::{
    config::structs::HoldMode,
    input::structs::{InputCommand, InputSender},
    ui::structs::{Keyboard, UiEvent},
};

use std::sync::{
    Arc,
    mpsc::{Receiver, Sender},
};

pub fn run_input_thread(
    rx_ic: Receiver<InputCommand>,
    tx_ue: Sender<UiEvent>,
    kb: Arc<Keyboard>,
    hm: HoldMode,
) {
    println!("THREAD START");

    let mut sender = InputSender::new(kb, tx_ue, hm).unwrap();

    while let Ok(cmd) = rx_ic.recv() {
        match cmd {
            InputCommand::KeyDown(key) => {
                println!("DOWN {:?}", key);
                sender.key_down(key).unwrap();
            }

            InputCommand::KeyUp(key) => {
                println!("UP {:?}", key);
                sender.key_up(key).unwrap();
            }
        }
    }
}
