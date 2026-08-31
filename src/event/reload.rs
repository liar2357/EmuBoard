use crate::{
    app::structs::{InputState, UiState},
    input::structs::InputCommand,
};
use gtk::Application;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock, mpsc::Sender},
};

pub fn reload_application(
    ui_state: &Rc<RefCell<UiState>>,
    input_state: &Arc<RwLock<InputState>>,
    app: &Application,
    tx_ic: &Sender<InputCommand>,
) {
    ui_state.borrow().window_close();

    let new_input_state = InputState::new();
    let new_ui_state = UiState::new(app, &new_input_state, tx_ic);

    *ui_state.borrow_mut() = new_ui_state;
    *input_state.write().unwrap() = new_input_state;
}
