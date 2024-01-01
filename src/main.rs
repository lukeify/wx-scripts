mod args;
mod sensors;

use sensors::{RainRadar, SensorTrait};
use clap::Parser;
use args::cli::Cli;
use args::resource::Resource;
use args::sensor_actions::SensorActions;

fn main() {
    match &Cli::parse().resource {
        Resource::Sensor(actions) => match_sensor_actions(actions),
        _ => panic!("Unimplemented resource.")
    }
}

fn match_sensor_actions(actions: &SensorActions) {
    match actions {
        SensorActions::List => {
            println!("RainRadar");
            println!("    Retrieves New Zealand rain radar imagery courtesy of MetService.")
        }
        // TODO: Conditionally instantiate/call the monitor method for the correct sensor based on the command
        //       line arguments provided.
        SensorActions::Monitor => RainRadar::monitor(),
        _ => panic!("Unimplemented action.")
    }
}