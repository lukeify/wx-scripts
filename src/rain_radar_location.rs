use std::fmt;

// derive Debug is necessary to use the name of the enum itself as a string
#[derive(Debug)]
pub enum RainRadarLocation {
    Auckland,
    BayOfPlenty,
    Christchurch,
    Invercargill,
    NewPlymouth,
    Northland,
    Mahia,
    Otago,
    Wellington,
    Westland
}

// These strings match up with the URL segment that is used to retrieve the radar image
// for the location.
impl fmt::Display for RainRadarLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RainRadarLocation::BayOfPlenty => write!(f, "Bay-of-Plenty"),
            RainRadarLocation::NewPlymouth => write!(f, "New-Plymouth"),
            _ => write!(f, "{:?}", self)
        }
    }
}