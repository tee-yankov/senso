use lm_sensors::{prelude::SharedChip, ChipRef, LMSensors};

use crate::sensors;

pub struct AppState {
    selected_chip: Option<i32>,
    sensors: LMSensors,
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
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            selected_chip: None,
            sensors: sensors::get_all_sensors().unwrap(),
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

    pub fn tick(&mut self) {}
}
