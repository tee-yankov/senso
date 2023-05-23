use std::iter::zip;

use itertools::Itertools;
use lm_sensors::{feature, prelude::SharedChip, value::Kind, ChipRef};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset},
    Frame, text::Span,
};

use crate::{app::App, utils::get_sub_feature};

use super::chip_list::ChipListProps;

pub fn get_temperature(chip: &ChipRef) -> Vec<(String, f64)> {
    chip.feature_iter()
        .filter(|feature| matches!(feature.kind(), Some(feature::Kind::Temperature)))
        .map(|feature| {
            let temperature: f64 = get_sub_feature(&feature, Kind::TemperatureInput).unwrap_or(0.0);

            (feature.label().unwrap(), temperature)
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
    let data = get_temperature(&chip);

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            data.iter()
                .map(|_| Constraint::Ratio(1, data.len() as u32))
                .collect::<Vec<Constraint>>(),
        )
        .split(area);

    charts(&app, f, &layout, props);
}

fn charts<B: Backend>(app: &App, f: &mut Frame<B>, layout: &[Rect], props: &ChipListProps) {
    let chip = if props.is_pinned_chip_view {
        app.state.get_pinned_chip().unwrap()
    } else {
        app.state.get_selected_chip()
    };
    for ((label, current_t), area) in zip(get_temperature(&chip).iter(), layout) {
        let feature = chip.feature_iter().find(|feature| {
            feature.label().unwrap() == *label
        }).unwrap();
        let max_t = get_sub_feature(&feature, Kind::TemperatureCritical).unwrap_or(100.0);
        let max_t_label = format!("{}C", &max_t);
        let half_max_t_label = format!("{}C", (max_t / 2.0).round());
        let current_t_pct_from_max = if max_t != 0.0 {
            100.min(((current_t / max_t) * 100.0) as u16)
        } else {
            *current_t as u16
        };
        // get historical data and combine with current_t
        let existing_temps = if let Some(existing_temps) = app.state.get_historical_data(label) {
            let mut v = Vec::from(existing_temps.buf.clone());
            v.push(*current_t);
            v
        } else {
            vec![*current_t]
        };
        let existing_temps: Vec<(f64, f64)> = existing_temps
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f64, *y))
            .collect();
        // x is index
        // y is temp
        let color = if current_t_pct_from_max < 50 {
            Color::Blue
        } else if current_t_pct_from_max < 80 {
            Color::Yellow
        } else {
            Color::Red
        };
        let dataset = Dataset::default()
            .name(label)
            .marker(symbols::Marker::Dot)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(color))
            .data(&existing_temps);
        let chart = Chart::new(vec![dataset])
            .block(Block::default().title(label.clone()))
            .x_axis(
                Axis::default()
                    .bounds([0.0, 100.0]),
            )
            .y_axis(
                Axis::default()
                    .title(format!("{}C", current_t))
                    .style(Style::default().fg(Color::White))
                    .labels(vec!["0C", &half_max_t_label, &max_t_label].into_iter().map(Span::from).collect_vec())
                    .bounds([0.0, if max_t == 0.0 { 100.0 } else { max_t }]),
            );

        f.render_widget(chart, *area);
    }
}
