use crate::{
    app::structs::{InputState, UiState},
    config::structs::UiPlace,
    input::structs::InputCommand,
    socket::structs::SocketCommand,
    ui::structs::{StyleCtl, UiEvent},
};
use gtk::{Application, glib::ControlFlow, prelude::*};
use gtk4_layer_shell::Edge;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc, RwLock,
        mpsc::{Receiver, Sender},
    },
};

pub fn socket_command_hundler(
    ui_state: &Rc<RefCell<UiState>>,
    input_state: &Arc<RwLock<InputState>>,
    app: &Application,
    rx_sc: &Receiver<SocketCommand>,
    tx_ic: &Sender<InputCommand>,
) -> ControlFlow {
    while let Ok(cmd) = rx_sc.try_recv() {
        match cmd {
            SocketCommand::ToggleUiView => ui_state
                .borrow()
                .window_set_visible(!ui_state.borrow().window_get_visible()),
            SocketCommand::ShowUiView => ui_state.borrow().window_set_visible(true),
            SocketCommand::HideUiView => ui_state.borrow().window_set_visible(false),
            SocketCommand::ToggleUiPlace => {
                let cc = ui_state.borrow().get_ui_place();

                match cc {
                    UiPlace::Lower => {
                        ui_state.borrow().window_set_anchor(Edge::Top);
                        ui_state.borrow_mut().set_ui_place(UiPlace::Upper);
                    }
                    UiPlace::Upper => {
                        ui_state.borrow().window_set_anchor(Edge::Bottom);
                        ui_state.borrow_mut().set_ui_place(UiPlace::Lower);
                    }
                }
            }
            SocketCommand::UpperUiPlace => {
                ui_state.borrow().window_set_anchor(Edge::Top);
                ui_state.borrow_mut().set_ui_place(UiPlace::Upper);
            }
            SocketCommand::LowerUiPlace => {
                ui_state.borrow().window_set_anchor(Edge::Bottom);
                ui_state.borrow_mut().set_ui_place(UiPlace::Lower);
            }
            SocketCommand::ReloadApp => {
                ui_state.borrow().window_close();

                let new_input_state = InputState::new();
                let new_ui_state = UiState::new(app, &new_input_state, tx_ic);

                *ui_state.borrow_mut() = new_ui_state;
                *input_state.write().unwrap() = new_input_state;
            }
            SocketCommand::ShutdownApp => app.quit(),
        }
    }

    ControlFlow::Continue
}

pub fn ui_event_hundler(ui_state: &Rc<RefCell<UiState>>, rx_ue: &Receiver<UiEvent>) -> ControlFlow {
    while let Ok(cmd) = rx_ue.try_recv() {
        match cmd {
            UiEvent::SetKeyText { pos, texts } => {
                ui_state
                    .borrow_mut()
                    .kct_set_text(pos, (&texts.0, &texts.1, &texts.2));
            }
            UiEvent::CtlKeyStyle { pos, mode, name } => match mode {
                StyleCtl::Add => {
                    ui_state.borrow_mut().kct_add_css_class(pos, name.as_str());
                }
                StyleCtl::Rmv => {
                    ui_state.borrow_mut().kct_rmv_css_class(pos, name.as_str());
                }
            },
        }
    }

    ControlFlow::Continue
}
