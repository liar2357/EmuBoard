use crate::{
    app::{
        handler::{reload_event_hundler, socket_command_hundler, ui_event_hundler},
        structs::{InputState, UiState},
        utils::bind_socket,
    },
    event::{
        hot_reload::{monitor_width, watch_file_change, watch_monitor_change},
        log::Logger,
        notify::send_notify,
        structs::{ReloadEvent, UiEvent},
    },
    input::{runner::run_input_thread, structs::InputCommand},
    socket::{handler::start_socket_server, structs::SocketCommand},
};
use gtk::{
    Application, gio,
    glib::{ExitCode, timeout_add_local},
    prelude::*,
};
use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, RwLock, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run(
    custom_config_path: Option<PathBuf>,
    join_hundlers: &mut Vec<JoinHandle<anyhow::Result<(), anyhow::Error>>>,
    logger: Arc<Logger>,
) -> ExitCode {
    if let Err(e) = gio::resources_register_include!("emu-board.gresource") {
        logger.error(format!("Failed to register resources\n{}", e));
        return ExitCode::FAILURE;
    }

    let custom_config_path = Rc::new(custom_config_path);

    let socket_path = format!(
        "{}/{}.sock",
        std::env::var("XDG_RUNTIME_DIR").unwrap(),
        env!("CARGO_PKG_NAME")
    );

    let listener = match bind_socket(&socket_path) {
        Ok(listener) => listener,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            send_notify("emu-board is already running.");
            logger.error("emu-board is already running.");
            return ExitCode::FAILURE;
        }
        Err(e) => {
            logger.error(e);
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

    let custom_config_path_c = Rc::clone(&custom_config_path);
    let input_state = InputState::new(&custom_config_path_c, Arc::clone(&logger));
    let input_state = Arc::new(RwLock::new(input_state));

    let input_state_c = Arc::clone(&input_state);
    let logger_c = Arc::clone(&logger);
    join_hundlers.push(thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, input_state_c, logger_c)
    }));

    let logger_c = Arc::clone(&logger);
    join_hundlers.push(thread::spawn(move || {
        start_socket_server(listener, tx_sc, spc, logger_c)
    }));

    let tx_re_c = tx_re.clone();
    let logger_c = Arc::clone(&logger);
    join_hundlers.push(thread::spawn(move || {
        watch_file_change(tx_re_c, rx_ss, logger_c)
    }));

    let tx_ic = RefCell::new(Some(tx_ic));
    let rx_sc = RefCell::new(Some(rx_sc));
    let rx_ue = RefCell::new(Some(rx_ue));
    let rx_re = RefCell::new(Some(rx_re));
    let tx_re = RefCell::new(Some(tx_re));

    let tx_ic_c = RefCell::clone(&tx_ic);

    let logger_c = Arc::clone(&logger);

    app.connect_activate(move |app| {
        let (tx_ic, rx_sc, rx_ue, rx_re, tx_re) = (
            tx_ic.borrow_mut().take().expect("activate called twice"),
            rx_sc.borrow_mut().take().expect("activate called twice"),
            rx_ue.borrow_mut().take().expect("activate called twice"),
            rx_re.borrow_mut().take().expect("activate called twice"),
            tx_re.borrow_mut().take().expect("activate called twice"),
        );

        let is_g = input_state.read().unwrap();
        let ui_state = UiState::new(app, &is_g, &tx_ic, Arc::clone(&logger_c));
        let ui_state = Rc::new(RefCell::new(ui_state));

        let app_c = app.clone();

        let ui_state_c = Rc::clone(&ui_state);
        let input_state_c = Arc::clone(&input_state);

        let tx_ic_c = tx_ic.clone();

        let logger_c1 = Arc::clone(&logger_c);
        let custom_config_path_c = Rc::clone(&custom_config_path);
        timeout_add_local(Duration::from_millis(16), move || {
            socket_command_hundler(
                &ui_state_c,
                &input_state_c,
                &app_c,
                &rx_sc,
                &tx_ic_c,
                &logger_c1,
                &custom_config_path_c,
            )
        });

        let ui_state_c = Rc::clone(&ui_state);
        timeout_add_local(Duration::from_millis(16), move || {
            ui_event_hundler(&ui_state_c, &rx_ue)
        });

        let app_c = app.clone();
        let input_state_c = Arc::clone(&input_state);
        let logger_c1 = Arc::clone(&logger_c);
        let custom_config_path_c = Rc::clone(&custom_config_path);
        timeout_add_local(Duration::from_millis(100), move || {
            reload_event_hundler(
                &ui_state,
                &input_state_c,
                &app_c,
                &tx_ic,
                &rx_re,
                &logger_c1,
                &custom_config_path_c,
            )
        });

        let monitor_name = input_state.read().unwrap().get_monitor_name();
        let logger_c1 = Arc::clone(&logger_c);
        let mut previous_width = monitor_width(&monitor_name, &logger_c1);

        timeout_add_local(Duration::from_millis(100), move || {
            watch_monitor_change(&monitor_name, &tx_re, &mut previous_width, &logger_c1)
        });
    });

    app.connect_shutdown(move |_| {
        let tx_ic = tx_ic_c.borrow_mut().take().expect("activate called twice");
        let _ = tx_ic.send(InputCommand::Shutdown);
        let _ = tx_ss.send(());
        let _ = std::fs::remove_file(&socket_path);
    });

    send_notify("Application Booted");
    logger.info("Application Booted");

    let argv = std::env::args()
        .next()
        .map(|arg| vec![arg])
        .unwrap_or_else(|| vec!["emu-board".to_string()]);

    app.run_with_args(&argv)
}
