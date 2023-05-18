use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lm_sensors::{feature, prelude::SharedChip, ChipRef};
use std::{cell::RefCell, error::Error};
use std::{thread, time::Duration};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{app::App, input::handle_input, terminal};

pub fn run_gui<'a>(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let mut terminal = terminal::get_terminal().unwrap();
    enable_raw_mode()?;
    let app = App::new();
    let input_poll_window = tick_rate;

    terminal.autoresize()?;

    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let app = RefCell::new(app);

    // Render Loop
    loop {
        terminal
            .draw(|f| {
                draw_ui(f, &app.borrow());
            })
            .unwrap();
        let event_available = event::poll(input_poll_window).unwrap();
        if event_available {
            if handle_input(&event::read().unwrap(), &app).is_err() {
                break;
            }
        }
        thread::sleep(tick_rate);
    }

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    Ok(())
}

fn draw_ui<'a, B: Backend>(f: &mut Frame<B>, app: &App) {
    let sensors = &app.state.get_sensors();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());
    let upper_block = Block::default().title("senso").borders(Borders::NONE);
    f.render_widget(upper_block, chunks[0]);

    let lower_block = Block::default().title("Sensors List").borders(Borders::ALL);
    let chip_list_items: Vec<ListItem> = sensors
        .chip_iter(None)
        .map(|chip| {
            chip_list_item(
                chip,
                chip.address() == app.state.get_selected_chip().address(),
            )
        })
        .collect();

    let list = List::new(chip_list_items).block(lower_block);

    f.render_widget(list, chunks[1]);
}

fn chip_list_item(chip: ChipRef, is_highlighted: bool) -> ListItem {
    let formatted_string = format!(
        "{}/{}",
        chip.prefix().unwrap().ok().unwrap(),
        chip.name().unwrap(),
    );
    let text = Text::from(formatted_string);
    let style = if is_highlighted {
        Some(Style {
            bg: Some(tui::style::Color::White),
            fg: Some(tui::style::Color::Black),
            ..Default::default()
        })
    } else {
        None
    };
    if let Some(style) = style {
        ListItem::new(text).style(style)
    } else {
        ListItem::new(text)
    }
}
