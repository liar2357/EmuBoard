use emu_board::socket::structs::SocketCommand;
use std::{env, io::Write, os::unix::net::UnixStream};

fn main() -> std::io::Result<()> {
    let Some(cmd) = env::args().nth(1) else {
        SocketCommand::print_all();
        eprintln!("--------------------");
        eprintln!("usage: emu-boardctl <command>");
        std::process::exit(1);
    };

    let socket = format!(
        "{}/{}.sock",
        std::env::var("XDG_RUNTIME_DIR").unwrap(),
        env!("CARGO_PKG_NAME")
    );

    let mut stream = UnixStream::connect(socket)?;
    writeln!(stream, "{cmd}")?;

    Ok(())
}
