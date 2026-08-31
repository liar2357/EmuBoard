use emu_board::{app::run::run, event::signal::run_signal_thread};
use gtk::glib::ExitCode;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::thread::{self, JoinHandle};

fn main() -> ExitCode {
    let mut join_hundlers: Vec<JoinHandle<anyhow::Result<(), anyhow::Error>>> = vec![];

    let mut signals = Signals::new([SIGINT]).unwrap();
    let handle = signals.handle();

    join_hundlers.push(thread::spawn(move || run_signal_thread(&mut signals)));

    let ec = run(&mut join_hundlers);

    handle.close();

    for jh in join_hundlers {
        if let Err(e) = jh.join() {
            eprintln!("{:?}", e);
        }
    }

    eprintln!("Successfuly Shutdown!");

    ec
}
