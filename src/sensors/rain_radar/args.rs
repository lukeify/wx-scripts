use crate::sensors::rain_radar::{Location, Range};

/// Defines the arguments accepted by the RainRadar Sensor when monitoring is initiated.
pub struct Args {
    /// The location of the rain radar to retrieve—this is one of ten locations in New Zealand
    /// where a radar facility is located.
    location: Location,
    /// The range of the radar to retrieve. This is either "300K" or "120K", the latter being a
    /// higher resolution image of a smaller area around the radar facility.
    range: Range,
}