use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::net::UnixListener,
    str::FromStr,
    sync::{Arc, mpsc::Sender},
};

use crate::{event::log::Logger, socket::structs::SocketCommand};

pub fn start_socket_server(
    listener: UnixListener,
    tx: Sender<SocketCommand>,
    socket_path: String,
    logger: Arc<Logger>,
) -> anyhow::Result<()> {
    logger.trace("Thread Steat: Socket");
    logger.info(format!("Listening: {}", socket_path));

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                logger.error(format!("Accept error: {e}"));
                continue;
            }
        };

        let mut reader = BufReader::new(stream);

        let mut line = String::new();

        if let Err(e) = reader.read_line(&mut line) {
            logger.error(format!("Read error: {e}"));
            continue;
        }

        let trimed = line.trim();
        logger.info(format!("Socket Recieved: {}", trimed));

        let cmd = match SocketCommand::from_str(trimed) {
            Ok(v) => v,
            Err(_) => {
                logger.error(format!("Unknown command: {}", line.trim()));
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

    logger.trace("Thread End: Socket");
    Ok(())
}
