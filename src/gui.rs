use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{cell::RefCell, error::Error};
use std::{thread, time::Duration};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::App,
    components::{
        chip_info::chip_info_panel,
        chip_list::{chip_list, ChipListProps},
        temperature_graphs::temperature_graphs,
    },
    input::handle_input,
    terminal,
};

pub fn run_gui(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    let mut terminal = terminal::get_terminal().unwrap();
    enable_raw_mode()?;
    let app = App::new();

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
        let mut should_break = false;
        while let Ok(event_available) = event::poll(Duration::from_millis(0)) {
            if event_available {
                if handle_input(&event::read().unwrap(), &app).is_err() {
                    should_break = true;
                    break;
                }
            } else {
                break;
            }
        }
        if should_break {
            break;
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
    let constraints = if let Some(_pinned_chip) = app.state.get_pinned_chip() {
        [
            Constraint::Percentage(6),
            Constraint::Percentage(47),
            Constraint::Percentage(47),
        ]
        .as_ref()
    } else {
        [Constraint::Percentage(6), Constraint::Percentage(94)].as_ref()
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(f.size());
    let key_binds_status_line = " | Pin (P/Enter) | Down (J/🠋) | Up (K/🠉)";
    let title_block = Block::default()
        .title(vec![
            Span::styled("♨️", Style::default().fg(ratatui::style::Color::Red)),
            Span::from(" senso "),
            Span::styled("♨️", Style::default().fg(ratatui::style::Color::Red)),
            key_binds_status_line.into(),
        ])
        .borders(Borders::NONE);
    f.render_widget(title_block, chunks[0]);

    if let Some(_) = app.state.get_pinned_chip() {
        draw_lower_block(
            f,
            app,
            chunks[1],
            ChipListProps {
                is_pinned_chip_view: false,
            },
        );
        draw_lower_block(
            f,
            app,
            chunks[2],
            ChipListProps {
                is_pinned_chip_view: true,
            },
        );
    } else {
        draw_lower_block(
            f,
            app,
            chunks[1],
            ChipListProps {
                is_pinned_chip_view: false,
            },
        );
    }
}

fn draw_lower_block<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect, props: ChipListProps) {
    // Left side sensor selection panel
    let nested_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(3, 12),
            Constraint::Ratio(3, 12),
            Constraint::Ratio(6, 12),
        ])
        .split(area);

    // Chip List
    chip_list(&app, f, nested_layout[0], &props);

    // Right side details panel
    chip_info_panel(&app, f, nested_layout[1], &props);

    // Charts
    temperature_graphs(&app, f, nested_layout[2], &props);
}
