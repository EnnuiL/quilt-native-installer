#![windows_subsystem = "windows"]

use anyhow::Result;
use clap::{Parser, Args, Subcommand};

mod gui;
mod installer;
pub mod theme;

const FONT_REGULAR: &'static [u8] = include_bytes!("../assets/poppins/Poppins-Regular.ttf");
const FONT_MEDIUM: &'static [u8] = include_bytes!("../assets/poppins/Poppins-Medium.ttf");
const FONT_SEMIBOLD: &'static [u8] = include_bytes!("../assets/poppins/Poppins-SemiBold.ttf");
const ICON: &'static [u8] = include_bytes!("../assets/quilt.png");

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Start the installer in no-gui mode
    #[arg(long)]
    no_gui: bool,
}

#[derive(Subcommand)]
enum Commands {
    Install(Install),
}

#[derive(Args)]
struct Install {
    #[command(subcommand)]
    command: InstallCommands
}

#[derive(Subcommand)]
enum InstallCommands {
    Client{},
    Server{},
}

fn main() -> Result<()> {
    gui::run();
    /*
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Install(install) => {
            match &install.command {
                InstallCommands::Client {  } => todo!(),
                InstallCommands::Server {  } => todo!(),
            }
        },
    }
    */

    Ok(())
}
