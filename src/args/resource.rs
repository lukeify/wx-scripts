use clap::Subcommand;
use crate::args::sensor_actions::SensorActions;

#[derive(Subcommand, Debug)]
pub enum Resource {
    #[command(subcommand)]
    Sensor(SensorActions),
    Product
}