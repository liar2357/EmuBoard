use crate::{
    app::{
        handler::{reload_event_hundler, socket_command_hundler, ui_event_hundler},
        structs::{InputState, UiState},
        utils::bind_socket,
    },
    event::{
        hot_reload::watch_file_change,
        notify::send_notify,
        structs::{ReloadEvent, UiEvent},
    },
    input::{runner::run_input_thread, structs::InputCommand},
    socket::{handler::start_socket_server, structs::SocketCommand},
};
use gtk::{Application, gio, glib::ExitCode, prelude::*};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run(join_hundlers: &mut Vec<JoinHandle<anyhow::Result<(), anyhow::Error>>>) -> ExitCode {
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
    let (tx_re, rx_re) = mpsc::channel::<ReloadEvent>();
    let (tx_ss, rx_ss) = mpsc::channel::<()>();

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    let spc = socket_path.clone();

    let input_state = InputState::new();
    let input_state = Arc::new(RwLock::new(input_state));

    let input_state_c = Arc::clone(&input_state);

    join_hundlers.push(thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, input_state_c)
    }));

    join_hundlers.push(thread::spawn(move || {
        start_socket_server(listener, tx_sc, spc)
    }));

    join_hundlers.push(thread::spawn(move || watch_file_change(tx_re, rx_ss)));

    let tx_ic = RefCell::new(Some(tx_ic));
    let rx_sc = RefCell::new(Some(rx_sc));
    let rx_ue = RefCell::new(Some(rx_ue));
    let rx_re = RefCell::new(Some(rx_re));

    let tx_ic_c = RefCell::clone(&tx_ic);

    app.connect_activate(move |app| {
        let tx_ic = tx_ic.borrow_mut().take().expect("activate called twice");
        let rx_sc = rx_sc.borrow_mut().take().expect("activate called twice");
        let rx_ue = rx_ue.borrow_mut().take().expect("activate called twice");
        let rx_re = rx_re.borrow_mut().take().expect("activate called twice");

        let is_g = input_state.read().unwrap();
        let ui_state = UiState::new(app, &is_g, &tx_ic);
        let ui_state = Rc::new(RefCell::new(ui_state));

        let app_c = app.clone();

        let ui_state_c = Rc::clone(&ui_state);
        let input_state_c = Arc::clone(&input_state);

        let tx_ic_c = tx_ic.clone();

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            socket_command_hundler(&ui_state_c, &input_state_c, &app_c, &rx_sc, &tx_ic_c)
        });

        let ui_state_c = Rc::clone(&ui_state);

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            ui_event_hundler(&ui_state_c, &rx_ue)
        });

        let app_c = app.clone();
        let input_state_c = Arc::clone(&input_state);

        gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
            reload_event_hundler(&ui_state, &input_state_c, &app_c, &tx_ic, &rx_re)
        });
    });

    app.connect_shutdown(move |_| {
        let tx_ic = tx_ic_c.borrow_mut().take().expect("activate called twice");
        let _ = tx_ic.send(InputCommand::Shutdown);
        let _ = tx_ss.send(());
        let _ = std::fs::remove_file(&socket_path);
    });

    send_notify("Application Booted");

    app.run()
}
