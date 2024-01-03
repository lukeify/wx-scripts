use clap::Subcommand;
use crate::args::sensor_monitor_args::SensorMonitorArgs;

#[derive(Subcommand, Debug)]
pub enum SensorActions {
    List,
    Monitor(SensorMonitorArgs),
    Cease,
    Status
}