use crate::sensors::rain_radar::RainRadarArgs;
use crate::sensors::rain_radar::RainRadarSensor;
use crate::sensors::SensorTrait;
use clap::Subcommand;

#[derive(Clone, Debug, Subcommand)]
pub enum Sensor {
    RainRadar(RainRadarArgs),
}

// TODO: Understand use of `+ '_` lifetime here.
impl Sensor {
    pub fn to_struct(&self) -> Box<dyn SensorTrait + '_> {
        match self {
            Sensor::RainRadar(args) => Box::new(RainRadarSensor { args }),
        }
    }
}
