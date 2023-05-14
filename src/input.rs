use std::{error::Error, io::{self, ErrorKind}};

use crossterm::event::{KeyCode, Event};

use crate::app::App;

pub fn handle_input<'a>(event: &Event, app: &'a mut App<'a>) -> Result<(), Box<dyn Error>> {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    Err(Box::new(io::Error::from(ErrorKind::Interrupted)))
                },
                KeyCode::Down => {
                    app.select_next_chip();
                    Ok(())
                },
                KeyCode::Up => {
                    app.select_previous_chip();
                    Ok(())
                }
                _ => Ok(())
            }
        },
        _ => Ok(())
    }
}
