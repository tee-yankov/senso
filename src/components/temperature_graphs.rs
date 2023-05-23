use std::iter::zip;

use lm_sensors::{feature, prelude::SharedChip, value::Kind, ChipRef, FeatureRef};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::app::App;

use super::chip_list::ChipListProps;

fn get_sub_feature(feature: &FeatureRef, kind: Kind) -> Option<f64> {
    feature
        .sub_feature_by_kind(kind)
        .iter()
        .map(|sub_feature| sub_feature.value().unwrap().raw_value())
        .next()
}

pub fn get_temperature_graph_data(chip: &ChipRef) -> Vec<(String, f64, f64)> {
    chip.feature_iter()
        .filter(|feature| matches!(feature.kind(), Some(feature::Kind::Temperature)))
        .map(|feature| {
            let temperature: f64 = get_sub_feature(&feature, Kind::TemperatureInput).unwrap_or(0.0);
            let max_temperature: f64 =
                get_sub_feature(&feature, Kind::TemperatureCritical).unwrap_or(0.0);

            (
                String::from(feature.label().unwrap()),
                temperature,
                max_temperature,
            )
        })
        .collect()
}

pub fn temperature_graphs<'a, B: Backend>(
    app: &App,
    f: &mut Frame<B>,
    area: Rect,
    props: &ChipListProps,
) {
    let chip = if props.is_pinned_chip_view {
        app.state.get_pinned_chip().unwrap()
    } else {
        app.state.get_selected_chip()
    };
    let data = get_temperature_graph_data(&chip);

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            data.iter()
                .map(|_| Constraint::Ratio(1, data.len() as u32))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

    let gauges = data.into_iter().map(|(name, current_t, max_t)| {
        let current_t_pct_from_max = if max_t != 0.0 {
            100.min(((current_t / max_t) * 100.0) as u16)
        } else {
            current_t as u16
        };
        let gauge_bg_color: Color = if current_t_pct_from_max > 50 {
            Color::Red
        } else {
            Color::Blue
        };
        let gauge_block = Block::default().title(name.clone()).borders(Borders::ALL);
        let label = if max_t != 0.0 {
            format!("{}C {}%", current_t, current_t_pct_from_max)
        } else {
            format!("{}C", current_t)
        };
        Gauge::default()
            .block(gauge_block)
            .label(label)
            .style(
                Style::default()
                    .fg(ratatui::style::Color::White)
                    .bg(gauge_bg_color),
            )
            .percent(current_t_pct_from_max)
    });

    for (gauge, layout_area) in zip(gauges, layout.iter()) {
        f.render_widget(gauge, *layout_area);
    }
}
