use lm_sensors::{prelude::SharedChip, ChipRef};
use ratatui::{Frame, layout::Rect, backend::Backend, widgets::{List, ListItem, Block, Borders}, text::Text, style::Style};

use crate::app::App;

pub struct ChipListProps {
    pub is_pinned_chip_view: bool,
}

pub fn chip_list<B: Backend>(app: &App, f: &mut Frame<B>, area: Rect, props: &ChipListProps) {
    let sensors = app.state.get_sensors();
    let lower_block = Block::default().title("Sensors List").borders(Borders::ALL);
    let selected_chip = if props.is_pinned_chip_view {
        app.state.get_pinned_chip().unwrap()
    } else {
        app.state.get_selected_chip()
    };
    let chip_list_items: Vec<ListItem> = sensors
        .chip_iter(None)
        .map(|chip| {
            chip_list_item(
                chip,
                chip.address() == selected_chip.address(),
            )
        })
        .collect();

    let list = List::new(chip_list_items).block(lower_block);
    f.render_widget(list, area);
}

pub fn chip_list_item(chip: ChipRef, is_highlighted: bool) -> ListItem {
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
