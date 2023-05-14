use std::{error::Error, io::{self, Stdout}};
use tui::{backend::CrosstermBackend, Terminal};

pub fn get_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}
