use clap::{Args};
use crate::sensors::Sensor;

#[derive(Args, Debug)]
pub struct SensorMonitorArgs {
    #[command(subcommand)]
    pub(crate) sensor: Sensor,
}

