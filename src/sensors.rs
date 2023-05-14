use lm_sensors::{Initializer, LMSensors};
use std::error::Error;

#[allow(dead_code)]
pub fn get_all_sensors() -> Result<LMSensors, Box<dyn Error>> {
    let sensors = Initializer::default().initialize()?;

    Ok(sensors)
}
