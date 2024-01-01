use clap::Args;
use crate::sensors::Sensor;

#[derive(Args)]
pub struct SensorMonitorArgs {
    sensor: Sensor,
}