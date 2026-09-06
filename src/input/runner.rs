use crate::{
    app::structs::InputState,
    event::{log::Logger, structs::UiEvent},
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
    logger: Arc<Logger>,
) -> anyhow::Result<()> {
    logger.trace("Thread Start: Input");

    let mut sender = InputSender::new(input_state, tx_ue, Arc::clone(&logger)).unwrap();

    while let Ok(cmd) = rx_ic.recv() {
        match cmd {
            InputCommand::KeyDown(key) => {
                logger.trace(format!("DOWN: {:?}", key));
                sender.key_down(key).unwrap();
            }

            InputCommand::KeyUp(key) => {
                logger.trace(format!("UP: {:?}", key));
                sender.key_up(key).unwrap();
            }

            InputCommand::Shutdown => break,
        }
    }

    logger.trace("Thread End: Input");
    Ok(())
}
