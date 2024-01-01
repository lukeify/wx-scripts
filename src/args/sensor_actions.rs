use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SensorActions {
    List,
    Monitor,
    Cease,
    Status
}