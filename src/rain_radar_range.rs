use std::fmt;

pub enum RainRadarRange {
    Wide,
    Close
}

impl fmt::Display for RainRadarRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RainRadarRange::Close => write!(f, "120K"),
            RainRadarRange::Wide => write!(f, "300K")
        }
    }
}

impl std::str::FromStr for RainRadarRange {
    type Err = ();

    fn from_str(input: &str) -> Result<RainRadarRange, Self::Err> {
        match input {
            "close" => Ok(RainRadarRange::Close),
            "wide" => Ok(RainRadarRange::Wide),
            _ => Err(())
        }
    }
}