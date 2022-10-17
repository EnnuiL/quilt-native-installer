#![windows_subsystem = "windows"]

use anyhow::Result;
use clap::Parser;

mod gui;
mod installer;
pub mod theme;

const FONT_REGULAR: &'static [u8] = include_bytes!("../assets/poppins/Poppins-Regular.ttf");
const FONT_MEDIUM: &'static [u8] = include_bytes!("../assets/poppins/Poppins-Medium.ttf");
const FONT_SEMIBOLD: &'static [u8] = include_bytes!("../assets/poppins/Poppins-SemiBold.ttf");
const ICON: &'static [u8] = include_bytes!("../assets/quilt.png");

#[derive(Default, Parser)]
#[clap(about, version)]
pub struct Args {
    /// Start the installer in no-gui mode
    #[clap(long)]
    no_gui: bool
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.no_gui {
        println!("No gui mode")
    } else {
        gui::run(args)?;
        //old_gui::run(args)?;
    }

    Ok(())
}
