use std::borrow::Borrow;

use lm_sensors::{ChipRef, LMSensors};

use crate::sensors;

pub struct AppState<'a> {
    selected_chip: Option<ChipRef<'a>>,
    sensors: LMSensors,
}

impl<'a> AppState<'a> {
    pub fn get_sensors(&self) -> &LMSensors {
        &self.sensors
    }

    pub fn get_selected_chip(&self) -> ChipRef {
        if let Some(selected_chip) = &self.selected_chip {
            *selected_chip.borrow()
        } else {
            self.sensors.chip_iter(None).next().unwrap()
        }
    }

    pub fn select_next_chip(&mut self) {
        let (current_chip_index, _) = self
            .sensors
            .chip_iter(None)
            .enumerate()
            .find(|(_, chip)| *chip == self.get_selected_chip())
            .unwrap();
        self.selected_chip = Some(
            self.sensors
                .chip_iter(None)
                .enumerate()
                .find(|(i, _)| *i == current_chip_index + 1)
                .unwrap()
                .1,
        );
    }

    pub fn select_previous_chip(&mut self) {
        let chips: Vec<_> = self.sensors.chip_iter(None).collect();
        let (current_chip_index, _) = self
            .sensors
            .chip_iter(None)
            .enumerate()
            .find(|(_, chip)| *chip == self.get_selected_chip())
            .unwrap();
        let (_, next_chip) = chips
            .iter()
            .enumerate()
            .rev()
            .find(|(i, _)| *i == current_chip_index - 1)
            .unwrap();
        self.selected_chip = None;
    }
}

impl<'a> Default for AppState<'a> {
    fn default() -> Self {
        AppState {
            selected_chip: None,
            sensors: sensors::get_all_sensors().unwrap(),
        }
    }
}

pub struct App<'a> {
    pub state: AppState<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            state: AppState::default(),
        }
    }
}
