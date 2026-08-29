use crate::socket::{sender::send_socket_command, structs::SocketCommand};
use signal_hook::{consts::SIGINT, iterator::SignalsInfo};

pub fn run_signal_thread(signals: &mut SignalsInfo) -> anyhow::Result<()> {
    eprintln!("Thread Start: Signal");
    for signal in signals.forever() {
        match signal {
            SIGINT => {
                eprintln!("Received Ctrl+C");
                let _ = send_socket_command(SocketCommand::ShutdownApp.to_string());
            }
            _ => unreachable!(),
        }
    }

    eprintln!("Thread End: Signal");
    Ok(())
}
