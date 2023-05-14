use std::{error::Error, time::Duration};

use app::App;
use gui::build_gui;

mod sensors;
mod terminal;
mod gui;
mod input;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = terminal::get_terminal().unwrap();

    build_gui(&mut term, Duration::from_millis(100))?;

    Ok(())
}
