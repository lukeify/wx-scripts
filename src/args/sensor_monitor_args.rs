use crate::sensors::Sensor;
use clap::Args;

#[derive(Args, Debug)]
pub struct SensorMonitorArgs {
    #[command(subcommand)]
    pub(crate) sensor: Sensor,
}
