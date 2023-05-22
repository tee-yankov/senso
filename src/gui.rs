use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{cell::RefCell, error::Error};
use std::{thread, time::Duration};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::App,
    components::{
        chip_info::chip_info_panel,
        temperature_graphs::temperature_graphs,
        chip_list::chip_list,
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
        let event_available = event::poll(Duration::from_millis(0)).unwrap();
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

    // Left side sensor selection panel
    let nested_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(3, 12),
            Constraint::Ratio(3, 12),
            Constraint::Ratio(6, 12),
        ])
        .split(chunks[1]);

    // Chip List
    chip_list(&app, f, nested_layout[0]);

    // Right side details panel
    chip_info_panel(&app, f, nested_layout[1]);

    // Charts
    temperature_graphs(&app, f, nested_layout[2]);
}
