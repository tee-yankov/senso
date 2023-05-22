use lm_sensors::{prelude::SharedChip, feature};
use ratatui::{widgets::{Paragraph, Block, Borders}, text::Text, backend::Backend, Frame, layout::Rect};

use crate::app::App;

use super::chip_list::ChipListProps;


pub fn chip_info_panel<B: Backend>(app: &App, f: &mut Frame<B>, area: Rect, props: &ChipListProps) {
    let chip = if props.is_pinned_chip_view {
        app.state.get_pinned_chip().unwrap()
    } else {
        app.state.get_selected_chip()
    };
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

    let paragraph = Paragraph::new(Text::from(feature_spans)).block(Block::default().borders(Borders::ALL).title("Sensor Details"));

    f.render_widget(paragraph, area)
}
