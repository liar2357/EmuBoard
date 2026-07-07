mod app;
mod input;
mod ui;

fn main() -> gtk::glib::ExitCode {
    crate::app::run()
}
