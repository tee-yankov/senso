use std::{error::Error, io::stdout, panic, time::Duration};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use gui::run_gui;
use logger::start_logger;

mod app;
mod gui;
mod input;
mod logger;
mod sensors;
mod terminal;
mod components;
mod ring_buffer;

fn main() -> Result<(), Box<dyn Error>> {
    start_logger();

    let _ = panic::catch_unwind(|| {
        run_gui(Duration::from_millis(100)).unwrap();
    });

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();

    Ok(())
}
