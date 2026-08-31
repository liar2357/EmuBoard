use crate::{
    app::structs::{InputState, UiState},
    config::structs::UiPlace,
    event::{
        notify::send_notify,
        reload::reload_application,
        structs::{ReloadEvent, UiEvent},
    },
    input::structs::InputCommand,
    socket::structs::SocketCommand,
    ui::structs::StyleCtl,
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
                reload_application(ui_state, input_state, app, tx_ic);
                send_notify("Application Reloaded");
            }
            SocketCommand::ShutdownApp => {
                send_notify("Application Shutdown");
                app.quit();
            }
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

pub fn reload_event_hundler(
    ui_state: &Rc<RefCell<UiState>>,
    input_state: &Arc<RwLock<InputState>>,
    app: &Application,
    tx_ic: &Sender<InputCommand>,
    rx_re: &Receiver<ReloadEvent>,
) -> ControlFlow {
    let mut is_reload_doing = false;

    while let Ok(eve) = rx_re.try_recv() {
        match eve {
            ReloadEvent::ChangeConfigFile => {
                is_reload_doing = true;
            }
        }
    }

    if is_reload_doing {
        reload_application(ui_state, input_state, app, tx_ic)
    }

    ControlFlow::Continue
}
