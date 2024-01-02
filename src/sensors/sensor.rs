use crate::sensors::rain_radar::RainRadarArgs;
use crate::sensors::rain_radar::RainRadar;
use clap::Subcommand;
use crate::sensors::SensorTrait;

#[derive(Clone, Debug, Subcommand)]
pub enum Sensor {
    RainRadar(RainRadarArgs)
}

impl Sensor {
    pub fn to_struct(&self) -> Box<dyn SensorTrait> {
        match self {
            Sensor::RainRadar(_) => Box::new(RainRadar {})
        }
    }
}