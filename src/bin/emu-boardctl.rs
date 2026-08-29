use emu_board::socket::{sender::send_socket_command, structs::SocketCommand};
use std::env;

fn main() -> std::io::Result<()> {
    let Some(cmd) = env::args().nth(1) else {
        SocketCommand::print_all();
        eprintln!("--------------------");
        eprintln!("usage: emu-boardctl <command>");
        std::process::exit(1);
    };

    send_socket_command(cmd)?;
    Ok(())
}
