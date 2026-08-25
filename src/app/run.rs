use crate::{
    app::{
        hundler::{socket_command_hundler, ui_event_hundler},
        structs::{InputState, UiState},
        utils::bind_socket,
    },
    input::{runner::run_input_thread, structs::InputCommand},
    socket::{hundler::start_socket_server, structs::SocketCommand},
    ui::structs::UiEvent,
};
use gtk::{Application, gio, glib::ExitCode, prelude::*};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock, mpsc},
    thread,
    time::Duration,
};

pub fn run() -> ExitCode {
    gio::resources_register_include!("emu-board.gresource").expect("Failed to register resources");

    let socket_path = format!(
        "{}/{}.sock",
        std::env::var("XDG_RUNTIME_DIR").unwrap(),
        env!("CARGO_PKG_NAME")
    );

    let listener = match bind_socket(&socket_path) {
        Ok(listener) => listener,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("emu-board is already running.");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let (tx_ic, rx_ic) = mpsc::channel::<InputCommand>();
    let (tx_sc, rx_sc) = mpsc::channel::<SocketCommand>();
    let (tx_ue, rx_ue) = mpsc::channel::<UiEvent>();

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    let spc = socket_path.clone();

    let input_state = InputState::new();
    let input_state = Arc::new(RwLock::new(input_state));

    let input_state_c = Arc::clone(&input_state);

    thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, input_state_c);
    });

    thread::spawn(move || start_socket_server(listener, tx_sc, spc));

    let tx_ic = RefCell::new(Some(tx_ic));
    let rx_sc = RefCell::new(Some(rx_sc));
    let rx_ue = RefCell::new(Some(rx_ue));

    app.connect_activate(move |app| {
        let tx_ic = tx_ic.borrow_mut().take().expect("activate called twice");
        let rx_sc = rx_sc.borrow_mut().take().expect("activate called twice");
        let rx_ue = rx_ue.borrow_mut().take().expect("activate called twice");

        let is_g = input_state.read().unwrap();
        let ui_state = UiState::new(app, &is_g, &tx_ic);
        let ui_state = Rc::new(RefCell::new(ui_state));

        let app_c = app.clone();

        let ui_state_c = Rc::clone(&ui_state);
        let input_state_c = Arc::clone(&input_state);

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            socket_command_hundler(&ui_state_c, &input_state_c, &app_c, &rx_sc, &tx_ic)
        });

        let ui_state_c = Rc::clone(&ui_state);

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            ui_event_hundler(&ui_state_c, &rx_ue)
        });
    });

    app.connect_shutdown(move |_| {
        let _ = std::fs::remove_file(&socket_path);
    });

    app.run()
}
