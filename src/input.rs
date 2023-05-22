use std::{error::Error, io::{self, ErrorKind}, cell::RefCell};

use crossterm::event::{KeyCode, Event};

use crate::app::App;

pub fn handle_input(event: &Event, app: &RefCell<App>) -> Result<(), Box<dyn Error>> {
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    Err(Box::new(io::Error::from(ErrorKind::Interrupted)))
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    app.borrow_mut().state.select_next_chip();
                    Ok(())
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    app.borrow_mut().state.select_previous_chip();
                    Ok(())
                },
                KeyCode::Enter | KeyCode::Char('p') => {
                    app.borrow_mut().state.set_pinned_chip();
                    Ok(())
                },
                _ => Ok(())
            }
        },
        _ => Ok(())
    }
}
