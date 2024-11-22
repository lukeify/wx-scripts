use clap::Args;
use crate::sensors::sensor::Sensor;

#[derive(Args)]
pub struct SensorMonitorArgs {
    sensor: Sensor,
}
