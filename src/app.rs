use std::collections::HashMap;

use lm_sensors::{prelude::SharedChip, ChipRef, LMSensors};

use crate::{ring_buffer::RingBuf, sensors};

pub struct AppState {
    selected_chip: Option<i32>,
    pinned_chip: Option<i32>,
    sensors: LMSensors,
    historical_data: HashMap<i32, RingBuf<(String, f64, f64)>>,
}

impl AppState {
    pub fn get_sensors(&self) -> &LMSensors {
        &self.sensors
    }

    pub fn get_selected_chip(&self) -> ChipRef {
        if let Some(selected_chip) = self.selected_chip {
            self.sensors
                .chip_iter(None)
                .find(|chip| chip.address().unwrap() == selected_chip)
                .unwrap()
        } else {
            self.sensors.chip_iter(None).next().unwrap()
        }
    }

    pub fn get_pinned_chip(&self) -> Option<ChipRef> {
        if let Some(pinned_chip) = self.pinned_chip {
            self.sensors
                .chip_iter(None)
                .find(|chip| chip.address().unwrap() == pinned_chip)
        } else {
            None
        }
    }

    fn get_nth_chip(&self, n: isize) -> ChipRef {
        let max_length: usize = self.sensors.chip_iter(None).collect::<Vec<ChipRef>>().len();
        let n = isize::min(isize::max(n, 0), (max_length - 1) as isize) as usize;
        let (_, next_chip) = self
            .sensors
            .chip_iter(None)
            .enumerate()
            .find(|(i, _)| *i == n)
            .unwrap();
        next_chip
    }

    fn get_current_chip_index(&self) -> usize {
        let (current_chip_index, _) = self
            .sensors
            .chip_iter(None)
            .enumerate()
            .find(|(_, chip)| *chip == self.get_selected_chip())
            .unwrap();
        current_chip_index
    }

    pub fn select_next_chip(&mut self) {
        let current_chip_index = self.get_current_chip_index();
        self.selected_chip = self
            .get_nth_chip((current_chip_index + 1) as isize)
            .address();
    }

    pub fn select_previous_chip(&mut self) {
        let current_chip_index = self.get_current_chip_index();
        self.selected_chip = self
            .get_nth_chip((current_chip_index.checked_sub(1).unwrap_or(0)) as isize)
            .address();
    }

    pub fn set_pinned_chip(&mut self) {
        if let Some(_) = self.pinned_chip {
            self.pinned_chip = None;
        } else {
            let selected_chip = self.get_selected_chip();
            self.pinned_chip = selected_chip.address();
        };
    }

    pub fn append_historical_data(&mut self, value: (String, f64, f64)) {
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            selected_chip: None,
            sensors: sensors::get_all_sensors().unwrap(),
            pinned_chip: None,
            historical_data: HashMap::new(),
        }
    }
}

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        App {
            state: AppState::default(),
        }
    }

    pub fn tick(&mut self) {
        let chip = self.state.get_selected_chip();
        // self.state.append_historical_data(get_temperature_graph_data(&chip));
    }
}
