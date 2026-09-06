use clap::{CommandFactory, FromArgMatches};
use emu_board::{
    event::comandline::Args4Ctl,
    socket::{sender::send_socket_command, structs::SocketCommand},
};

fn main() -> anyhow::Result<()> {
    let mut command = Args4Ctl::command();

    let available_commands = SocketCommand::get_all_comands_string()
        .lines()
        .map(|line| format!("  {line}"))
        .collect::<Vec<_>>()
        .join("\n");

    command = command.after_help(format!("Available commands:\n{available_commands}"));

    let args = Args4Ctl::from_arg_matches(&command.get_matches())?;

    let Some(cmd) = args.command else {
        SocketCommand::print_all();
        eprintln!("--------------------");
        eprintln!("usage: emu-boardctl <command>");
        std::process::exit(1);
    };

    send_socket_command(cmd)?;

    Ok(())
}
