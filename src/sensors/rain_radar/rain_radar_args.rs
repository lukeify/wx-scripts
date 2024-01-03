use clap::Args;
use crate::sensors::rain_radar::{Location, Range};

/// Defines the arguments accepted by the RainRadar Sensor when monitoring is initiated.
// TODO: Understand use of `pub(crate)`
#[derive(Clone, Debug, Args)]
pub struct RainRadarArgs {
    /// The location of the rain radar to retrieve—this is one of ten locations in New Zealand
    /// where a radar facility is located.
    #[arg(long)]
    pub(crate) location: Location,
    /// The range of the radar to retrieve. This is either "300K" or "120K", the latter being a
    /// higher resolution image of a smaller area around the radar facility.
    #[arg(long)]
    pub(crate) range: Range,
}