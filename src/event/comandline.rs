use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "emu-board")]
#[command(version)]
#[command(about = "GTK4-based on-screen keyboard for Wayland")]
pub struct Args {
    /// Configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Parser, Debug)]
#[command(
    name = "emu-boardctl",
    version,
    about = "Control a running EmuBoard instance."
)]
pub struct Args4Ctl {
    pub command: Option<String>,
}
