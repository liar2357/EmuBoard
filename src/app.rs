use crate::ui::builder::build_ui;
use gtk::Application;
use gtk::prelude::*;

pub fn run() -> gtk::glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.keyboard")
        .build();

    app.connect_activate(build_ui);
    app.run()
}
