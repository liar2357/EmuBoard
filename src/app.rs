use crate::{
    config::{load::load_config, structs::Config},
    input::{runner::run_input_thread, structs::InputCommand},
    ui::{
        builder::build_ui,
        load::load_keyboard,
        structs::{KeyComponentsTable, Keyboard, StyleCtl, UiEvent},
    },
};
use gtk::Application;
use gtk::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

pub fn run() -> gtk::glib::ExitCode {
    let config: Config = load_config();
    eprintln!("{:?}", config);

    let keyboard: Arc<Keyboard> = Arc::new(load_keyboard(&config.layout));

    let (tx_ic, rx_ic) = mpsc::channel::<InputCommand>();
    let (tx_ue, rx_ue) = mpsc::channel::<UiEvent>();
    let rx_ue = RefCell::new(Some(rx_ue));

    let arc_kb = Arc::clone(&keyboard);

    thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, arc_kb, config.hold_mode);
    });

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    app.connect_activate(move |app| {
        let rx_ue = rx_ue.borrow_mut().take().expect("activate called twice");

        let kct = Rc::new(RefCell::new(KeyComponentsTable::new()));

        build_ui(
            app,
            Arc::clone(&keyboard),
            &mut kct.borrow_mut(),
            tx_ic.clone(),
            &config.default_monitor,
        );

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

    app.run()
}
