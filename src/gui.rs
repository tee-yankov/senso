use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lm_sensors::{feature, prelude::SharedChip, ChipRef};
use std::{cell::RefCell, error::Error};
use std::{thread, time::Duration};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    symbols,
    text::{Span, Text},
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, input::handle_input, terminal};

pub fn run_gui(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
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
        app.borrow_mut().tick();
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

fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let sensors = &app.state.get_sensors();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)].as_ref())
        .split(f.size());
    let upper_block = Block::default()
        .title(vec![
            Span::styled("♨️", Style::default().fg(ratatui::style::Color::Red)),
            Span::from(" senso "),
            Span::styled("♨️", Style::default().fg(ratatui::style::Color::Red)),
        ])
        .borders(Borders::NONE);
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

    // Left side sensor selection panel
    let nested_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(chunks[1]);
    let list = List::new(chip_list_items).block(lower_block);
    f.render_widget(list, nested_layout[0]);

    // Right side details panel
    let chip_details: Paragraph = chip_info_panel(app.state.get_selected_chip());
    f.render_widget(chip_details, nested_layout[1]);

    // Charts
    let chart = temperature_graphs(app.state.get_graph_data());
    f.render_widget(chart, nested_layout[2]);
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
            bg: Some(ratatui::style::Color::White),
            fg: Some(ratatui::style::Color::Black),
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

fn chip_info_panel(chip: ChipRef) -> Paragraph {
    let feature_spans = chip
        .feature_iter()
        .filter_map(|feature| {
            if feature.kind() == Some(feature::Kind::Temperature) {
                let temperatures: String = feature
                    .sub_feature_iter()
                    .filter_map(|sub_feature| {
                        if let (Some(Ok(name)), Ok(value)) =
                            (sub_feature.name(), sub_feature.value())
                        {
                            Some(format!("\n [{} {}]", name, value))
                        } else {
                            None
                        }
                    })
                    .collect();
                Some(format!(" {} {}\n", feature.label().unwrap(), temperatures))
            } else {
                None
            }
        })
        .collect::<String>();

    Paragraph::new(Text::from(feature_spans))
}

fn temperature_graphs(graph_data: &[(f64, f64)]) -> Chart {
    let dataset = Dataset::default()
        .name("temperature")
        .marker(symbols::Marker::Block)
        .graph_type(ratatui::widgets::GraphType::Line)
        .style(Style::default().fg(ratatui::style::Color::Cyan))
        .data(graph_data);

    Chart::new(vec![dataset])
        .block(Block::default().title("Temperature Chart").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title(Span::styled(
                    "X Axis",
                    Style::default().fg(ratatui::style::Color::Red),
                ))
                .bounds([0.0, 100.0]),
        )
        .y_axis(
            Axis::default()
                .title(Span::styled(
                    "Y Axis",
                    Style::default().fg(ratatui::style::Color::White),
                ))
                .bounds([0.0, 100.0]),
        )
}
