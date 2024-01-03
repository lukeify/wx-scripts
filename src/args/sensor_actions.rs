use crate::args::sensor_monitor_args::SensorMonitorArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SensorActions {
    List,
    Monitor(SensorMonitorArgs),
    Cease,
    Status,
}
