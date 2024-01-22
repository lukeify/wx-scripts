mod args;
mod database;
mod sensors;

use args::cli::Cli;
use args::resource::Resource;
use args::sensor_actions::SensorActions;
use clap::Parser;

fn main() {
    match &Cli::parse().resource {
        Resource::Sensor(action) => match_sensor_action(action),
        Resource::Product => panic!("Unimplemented resource."),
    }
}

/// If the CLI tool is handling a sensor action, this function will be called.
///
/// # Arguments
///
/// * `action` - The `SensorAction` that was called alongside the "sensor" command.
fn match_sensor_action(action: &SensorActions) {
    match action {
        SensorActions::List => {
            println!("RainRadar");
            println!("    Retrieves New Zealand rain radar imagery courtesy of MetService.");
        }
        SensorActions::Monitor(args) => args.sensor.to_struct().monitor(),
        _ => panic!("Unimplemented action."),
    }
}
