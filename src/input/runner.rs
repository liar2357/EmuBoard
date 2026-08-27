use crate::{
    app::structs::InputState,
    event::structs::UiEvent,
    input::structs::{InputCommand, InputSender},
};

use std::sync::{
    Arc, RwLock,
    mpsc::{Receiver, Sender},
};

pub fn run_input_thread(
    rx_ic: Receiver<InputCommand>,
    tx_ue: Sender<UiEvent>,
    input_state: Arc<RwLock<InputState>>,
) {
    println!("THREAD START");

    let mut sender = InputSender::new(input_state, tx_ue).unwrap();

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
