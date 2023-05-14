use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lm_sensors::{feature, prelude::SharedChip, ChipRef, LMSensors};
use std::{error::Error, rc::Rc, cell::RefCell, thread::panicking};
use std::{io::Write, thread, time::Duration};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::{app::App, input::handle_input};

pub fn build_gui<'a, T: Backend + Write>(
    terminal: &mut Terminal<T>,
    tick_rate: Duration,
) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut app = App::new();

    terminal.autoresize()?;

    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let input_poll_window = Duration::from_millis(16);

    loop {
        terminal.draw(|f| {
            draw_ui(f, &app.get_state().get_sensors(), &app);
        })?;

        let event_available = event::poll(input_poll_window).unwrap();
        if event_available {
            if handle_input(&event::read().unwrap(), &mut app).is_err() {
                disable_raw_mode().unwrap();
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                ).unwrap();
                terminal.show_cursor().unwrap();
                break;
            }
        }

        thread::sleep(tick_rate - input_poll_window);
    }

    Ok(())
}

fn draw_ui<'a, B: Backend>(f: &mut Frame<B>, sensors: &LMSensors, app: &'a App<'a>) {
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
            chip_list_item(chip, chip.address() == app.get_state().get_selected_chip().unwrap().address())
        })
        .collect();
    let list = List::new(chip_list_items).block(lower_block);

    f.render_widget(list, chunks[1]);
}

fn chip_list_item(chip: ChipRef, is_highlighted: bool) -> ListItem {
    let formatted_string = format!(
        "{}/{} {}",
        chip.prefix().unwrap().ok().unwrap(),
        chip.name().unwrap(),
        chip.feature_iter()
            .filter_map(|feature| {
                if feature.kind() == Some(feature::Kind::Temperature) {
                    let temperatures: String = feature
                        .sub_feature_iter()
                        .filter_map(|sub_feature| {
                            if let (Some(Ok(name)), Ok(value)) =
                                (sub_feature.name(), sub_feature.value())
                            {
                                Some(format!("[{} {}]", name, value))
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(format!(" {} {}", feature.label().unwrap(), temperatures))
                } else {
                    None
                }
            })
            .collect::<String>()
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