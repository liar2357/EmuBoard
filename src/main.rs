use clap::Parser;
use emu_board::{
    app::run::run,
    event::{comandline::Args, log::Logger, signal::run_signal_thread},
};
use gtk::glib::ExitCode;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

fn main() -> ExitCode {
    let args = Args::parse();
    let logger = Arc::new(Logger::new(args.verbose));

    let mut join_hundlers: Vec<JoinHandle<anyhow::Result<(), anyhow::Error>>> = vec![];

    let mut signals = Signals::new([SIGINT]).unwrap();
    let handle = signals.handle();

    let logger_c = Arc::clone(&logger);
    join_hundlers.push(thread::spawn(move || {
        run_signal_thread(&mut signals, logger_c)
    }));

    let ec = run(args.config, &mut join_hundlers, Arc::clone(&logger));

    handle.close();

    for jh in join_hundlers {
        if let Err(e) = jh.join() {
            logger.error(format!("{:?}", e));
        }
    }

    logger.info("Successfuly Shutdown!");

    ec
}
