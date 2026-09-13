use crate::{
    app::structs::{InputState, UiState},
    event::log::Logger,
    input::structs::InputCommand,
};
use gtk::Application;
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, RwLock, mpsc::Sender},
};

pub fn reload_application(
    custom_path: &Option<PathBuf>,
    ui_state: &Rc<RefCell<UiState>>,
    input_state: &Arc<RwLock<InputState>>,
    app: &Application,
    tx_ic: &Sender<InputCommand>,
    logger: Arc<Logger>,
) {
    ui_state.borrow().window_close();

    let new_input_state = InputState::new(custom_path, Arc::clone(&logger));
    let new_ui_state = UiState::new(app, &new_input_state, tx_ic, Arc::clone(&logger));

    *ui_state.borrow_mut() = new_ui_state;
    *input_state.write().unwrap() = new_input_state;
}

pub fn switch_profile(
    ui_state: &Rc<RefCell<UiState>>,
    input_state: &Arc<RwLock<InputState>>,
    app: &Application,
    tx_ic: &Sender<InputCommand>,
    logger: Arc<Logger>,
    idx: usize,
) {
    ui_state.borrow().window_close();

    let new_ui_state = UiState::new(
        app,
        input_state.write().unwrap().switch_profile(idx),
        tx_ic,
        Arc::clone(&logger),
    );

    *ui_state.borrow_mut() = new_ui_state;
}
