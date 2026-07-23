use crate::{
    config::{load::load_config, structs::Config},
    input::{runner::run_input_thread, structs::InputCommand},
    ui::{
        builder::build_ui,
        load::load_keyboard,
        structs::{KeyLabelTable, Keyboard, UiEvent},
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

    let keyboard: Arc<Keyboard> = Arc::new(load_keyboard(&config.layout));

    let (tx_ic, rx_ic) = mpsc::channel::<InputCommand>();
    let (tx_ue, rx_ue) = mpsc::channel::<UiEvent>();
    let rx_ue = RefCell::new(Some(rx_ue));

    let arc_kb = Arc::clone(&keyboard);

    thread::spawn(move || {
        run_input_thread(rx_ic, tx_ue, arc_kb);
    });

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    app.connect_activate(move |app| {
        let rx_ue = rx_ue.borrow_mut().take().expect("activate called twice");

        let klt = Rc::new(RefCell::new(KeyLabelTable::new()));

        build_ui(
            app,
            Arc::clone(&keyboard),
            &mut klt.borrow_mut(),
            tx_ic.clone(),
            &config.default_monitor,
        );

        let klt_for_timer = Rc::clone(&klt);

        gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
            while let Ok(cmd) = rx_ue.try_recv() {
                match cmd {
                    UiEvent::SetKeyText { pos, texts } => {
                        klt_for_timer
                            .borrow_mut()
                            .set_text(pos, (&texts.0, &texts.1, &texts.2));
                    }
                }
            }

            gtk::glib::ControlFlow::Continue
        });
    });

    app.run()
}
