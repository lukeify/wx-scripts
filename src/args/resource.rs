use crate::args::sensor_actions::SensorActions;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Resource {
    #[command(subcommand)]
    Sensor(SensorActions),
    Product,
}
