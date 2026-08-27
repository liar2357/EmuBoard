use emu_board::app::run::run;
use gtk::glib::ExitCode;
use std::thread::JoinHandle;

fn main() -> ExitCode {
    let mut join_hundlers: Vec<JoinHandle<anyhow::Result<(), anyhow::Error>>> = vec![];

    let ec = run(&mut join_hundlers);

    for jh in join_hundlers {
        if let Err(e) = jh.join() {
            eprintln!("{:?}", e);
        }
    }

    ec
}
