use lm_sensors::{ChipRef, LMSensors};

use crate::sensors;

pub struct AppState<'a> {
    selected_chip: Option<ChipRef<'a>>,
    sensors: LMSensors,
}

impl<'a> AppState<'a> {
    pub fn get_sensors(&'a self) -> &'a LMSensors {
        &self.sensors
    }

    pub fn get_selected_chip(&'a self) -> Option<&'a ChipRef<'a>> {
        if let Some(selected_chip) = &self.selected_chip {
            Some(selected_chip)
        } else {
            None
        }
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

    pub fn get_state(&'a self) -> &'a AppState<'a> {
        &self.state
    }

    pub fn select_next_chip(&'a mut self) {
        unimplemented!()
    }

    pub fn select_previous_chip(&'a mut self) {
        unimplemented!()
    }
}
