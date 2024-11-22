use clap::Parser;
use crate::args::resource::Resource;

#[derive(Parser)]
#[command(name = "wx-scripts")]
#[command(author = "lukeify <5379845+lukeify@users.noreply.github.com>")]
#[command(version = "0.0.1")]
#[command(about = "Helpful weather scripts.")]
pub struct Cli {
    /// The resource being controlled.
    #[command(subcommand)]
    pub(crate) resource: Resource
}
