mod app;
mod config;
mod input;
mod ui;

fn main() -> gtk::glib::ExitCode {
    crate::app::run()
}
