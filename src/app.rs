use crate::{
    config::{
        load::load_config,
        structs::{Config, UiPlace},
    },
    input::{runner::run_input_thread, structs::InputCommand},
    socket::{hundler::start_socket_server, structs::SocketCommand},
    ui::{
        builder::build_ui,
        load::load_keyboard,
        structs::{KeyComponentsTable, Keyboard, StyleCtl, UiEvent},
    },
};
use gtk::{Application, gio, glib::ExitCode, prelude::*};
use gtk4_layer_shell::{Edge, LayerShell};
use std::{
    cell::RefCell,
    fs, io,
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    rc::Rc,
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

pub fn bind_socket(socket_path: &str) -> io::Result<UnixListener> {
    // ソケットファイルが残っているか
    if Path::new(socket_path).exists() {
        // 接続できるなら既に起動中
        if UnixStream::connect(socket_path).is_ok() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Application is already running",
            ));
        }

        // 接続できなければ古いソケットなので削除
        fs::remove_file(socket_path)?;
    }

    UnixListener::bind(socket_path)
}

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

    let config: Config = load_config();
    eprintln!("{:?}", config);

    let keyboard: Arc<Keyboard> = Arc::new(load_keyboard(&config.layout));

    let (tx_ic, rx_ic) = mpsc::channel::<InputCommand>();
    let (tx_sc, rx_sc) = mpsc::channel::<SocketCommand>();
    let (tx_ue, rx_ue) = mpsc::channel::<UiEvent>();

    let rx_sc = RefCell::new(Some(rx_sc));
    let rx_ue = RefCell::new(Some(rx_ue));

    let arc_kb = Arc::clone(&keyboard);

    let spc = socket_path.clone();
    thread::spawn(move || start_socket_server(listener, tx_sc, spc));

    thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, arc_kb, config.hold_mode);
    });

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    app.connect_activate(move |app| {
        let rx_sc = rx_sc.borrow_mut().take().expect("activate called twice");
        let rx_ue = rx_ue.borrow_mut().take().expect("activate called twice");

        let kct = Rc::new(RefCell::new(KeyComponentsTable::new()));

        let app_c = app.clone();

        let window = build_ui(
            &app_c,
            Arc::clone(&keyboard),
            &mut kct.borrow_mut(),
            tx_ic.clone(),
            &config.default_monitor,
            &config.default_ui_view,
            &config.default_ui_place,
        );

        let app_c = app.clone();
        let current_ui_place = Rc::new(RefCell::new(config.default_ui_place.clone()));

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            while let Ok(cmd) = rx_sc.try_recv() {
                match cmd {
                    SocketCommand::ToggleUiView => window.set_visible(!window.is_visible()),
                    SocketCommand::ShowUiView => window.set_visible(true),
                    SocketCommand::HideUiView => window.set_visible(false),
                    SocketCommand::ToggleUiPlace => {
                        let cc = current_ui_place.borrow().clone();

                        match cc {
                            UiPlace::Lower => {
                                window.set_anchor(Edge::Bottom, false);
                                window.set_anchor(Edge::Top, true);
                                *current_ui_place.borrow_mut() = UiPlace::Upper;
                            }
                            UiPlace::Upper => {
                                window.set_anchor(Edge::Top, false);
                                window.set_anchor(Edge::Bottom, true);
                                *current_ui_place.borrow_mut() = UiPlace::Lower;
                            }
                        }
                    }
                    SocketCommand::UpperUiPlace => {
                        window.set_anchor(Edge::Bottom, false);
                        window.set_anchor(Edge::Top, true);
                        *current_ui_place.borrow_mut() = UiPlace::Upper;
                    }
                    SocketCommand::LowerUiPlace => {
                        window.set_anchor(Edge::Top, false);
                        window.set_anchor(Edge::Bottom, true);
                        *current_ui_place.borrow_mut() = UiPlace::Lower;
                    }
                    SocketCommand::ShutdownApp => app_c.quit(),
                }
            }

            gtk::glib::ControlFlow::Continue
        });

        let kct_for_timer = Rc::clone(&kct);

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            while let Ok(cmd) = rx_ue.try_recv() {
                match cmd {
                    UiEvent::SetKeyText { pos, texts } => {
                        kct_for_timer
                            .borrow_mut()
                            .set_text(pos, (&texts.0, &texts.1, &texts.2));
                    }
                    UiEvent::CtlKeyStyle { pos, mode, name } => match mode {
                        StyleCtl::Add => {
                            kct_for_timer.borrow_mut().add_css_class(pos, name.as_str());
                        }
                        StyleCtl::Rmv => {
                            kct_for_timer.borrow_mut().rmv_css_class(pos, name.as_str());
                        }
                    },
                }
            }

            gtk::glib::ControlFlow::Continue
        });
    });

    app.connect_shutdown(move |_| {
        let _ = std::fs::remove_file(&socket_path);
    });

    app.run()
}
