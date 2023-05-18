use std::{error::Error, io::stdout, panic, time::Duration};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use gui::run_gui;
use logger::{log_message, start_logger};

mod app;
mod gui;
mod input;
mod logger;
mod sensors;
mod terminal;

fn main() -> Result<(), Box<dyn Error>> {
    start_logger();

    log_message("-==NEW RUN==-");

    let _ = panic::catch_unwind(|| {
        run_gui(Duration::from_millis(160)).unwrap();
    });

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();

    Ok(())
}
