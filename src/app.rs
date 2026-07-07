use crate::{
    input::{
        funcs::{run_input_thread, supported_keys},
        structs::InputCommand,
    },
    ui::{builder::build_ui, load::load_keyboard, structs::Keyboard},
};
use gtk::Application;
use gtk::prelude::*;
use std::sync::mpsc;
use std::thread;

pub fn run() -> gtk::glib::ExitCode {
    let keyboard: Keyboard = load_keyboard("JIS-QWERTY");
    let keys = supported_keys(&keyboard);

    let (tx, rx) = mpsc::channel::<InputCommand>();

    thread::spawn(move || {
        run_input_thread(rx, keys);
    });

    let app = Application::builder()
        .application_id("com.EmuBoard.emuboard")
        .build();

    app.connect_activate(move |app| {
        build_ui(app, &keyboard, tx.clone());
    });

    app.run()
}
