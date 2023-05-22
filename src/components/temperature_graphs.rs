use std::iter::zip;

use lm_sensors::{feature, prelude::SharedChip};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Gauge, Block, Borders},
    Frame,
};

use crate::app::App;

pub fn temperature_graphs<'a, B: Backend>(app: &App, f: &mut Frame<B>, area: Rect) {
    let chip = app.state.get_selected_chip();
    let data: Vec<(String, f64)> = chip
        .feature_iter()
        .filter(|feature| matches!(feature.kind(), Some(feature::Kind::Temperature)))
        .map(|feature| {
            let temperatures: Vec<f64> = feature
                .sub_feature_by_kind(lm_sensors::value::Kind::TemperatureInput)
                .iter()
                .map(|sub_feature| sub_feature.value().unwrap().raw_value())
                .collect();

            (
                String::from(feature.label().unwrap()),
                *temperatures.get(0).unwrap_or(&0.0),
            )
        })
        .collect();

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            data.iter()
                .map(|_| Constraint::Ratio(1, data.len() as u32))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

    let gauges = data.into_iter().map(|(name, current_t)| {
        let gauge_block = Block::default().title(name.clone()).borders(Borders::ALL);
        Gauge::default()
            .block(gauge_block)
            .label(format!("{}C", current_t))
            .style(
                Style::default()
                    .fg(ratatui::style::Color::Red)
                    .bg(ratatui::style::Color::Cyan),
            )
            .percent(current_t as u16)
    });

    for (gauge, layout_area) in zip(gauges, layout.iter()) {
        f.render_widget(gauge, *layout_area);
    }
}
