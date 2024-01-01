use clap::Parser;
use crate::args::resource::Resource;

/// Defines the entrypoint for the command line interface for the `wx-scripts` tool.
#[derive(Parser)]
#[command(name = "wx-scripts")]
#[command(author = "lukeify <5379845+lukeify@users.noreply.github.com>")]
#[command(version = "0.0.1")]
#[command(about = "Helpful weather scripts.")]
pub struct Cli {
    /// The resource that is being actioned. This may be a sensor, or a product.
    #[command(subcommand)]
    pub(crate) resource: Resource
}