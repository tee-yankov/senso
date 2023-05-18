use std::{error::Error, time::Duration, panic, io::stdout};

use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen}, execute, event::DisableMouseCapture};
use gui::run_gui;

mod sensors;
mod terminal;
mod gui;
mod input;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = panic::catch_unwind(|| {
        run_gui(Duration::from_millis(16)).unwrap();
    });

    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();

    Ok(())
}
