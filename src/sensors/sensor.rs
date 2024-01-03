use crate::sensors::rain_radar::RainRadarArgs;
use crate::sensors::rain_radar::RainRadar;
use clap::Subcommand;
use crate::sensors::SensorTrait;

#[derive(Clone, Debug, Subcommand)]
pub enum Sensor {
    RainRadar(RainRadarArgs)
}

// TODO: Understand use of `+ '_` lifetime here.
impl Sensor {
    pub fn to_struct(&self) -> Box<dyn SensorTrait + '_> {
        match self {
            Sensor::RainRadar(args) => Box::new(RainRadar { args })
        }
    }
}