use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::net::UnixListener,
    str::FromStr,
    sync::mpsc::Sender,
};

use crate::socket::structs::SocketCommand;

pub fn start_socket_server(
    listener: UnixListener,
    tx: Sender<SocketCommand>,
    socket_path: String,
) -> anyhow::Result<()> {
    eprintln!("Thread Steat: Socket");
    eprintln!("Listening: {}", socket_path);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Accept error: {e}");
                continue;
            }
        };

        let mut reader = BufReader::new(stream);

        let mut line = String::new();

        if let Err(e) = reader.read_line(&mut line) {
            eprintln!("Read error: {e}");
            continue;
        }

        let cmd = match SocketCommand::from_str(line.trim()) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Unknown command: {}", line.trim());
                continue;
            }
        };

        let is_shutdown = matches!(cmd, SocketCommand::ShutdownApp);

        if tx.send(cmd).is_err() {
            break;
        }

        if is_shutdown {
            break;
        }
    }

    let _ = fs::remove_file(&socket_path);

    eprintln!("Thread End: Socket");
    Ok(())
}
