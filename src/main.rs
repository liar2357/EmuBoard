mod app;
mod ui;

fn main() -> gtk::glib::ExitCode {
    crate::app::run()
}
