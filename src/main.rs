use std::{error::Error, io::stdout, panic, time::Duration};

use clap::{arg, command, Parser};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use gui::run_gui;
use logger::{start_logger, log_message};

mod app;
mod components;
mod gui;
mod input;
mod logger;
mod ring_buffer;
mod sensors;
mod terminal;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    tick_rate: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    start_logger();

    let args = Args::parse();
    log_message(&format!("tick_rate = {}", args.tick_rate));

    let _ = panic::catch_unwind(|| {
        run_gui(Duration::from_millis(args.tick_rate as u64)).unwrap();
    });

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();

    Ok(())
}
