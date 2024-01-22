use clap::ValueEnum;
use std::fmt;
use serde::Serialize;

/// The `Debug` item for the derive attribute is necessary to use the name of the enum as a string.
/// The `ValueEnum` item is needed because this enum is used as a command line argument.
/// TODO: Understand clone and copy.
#[derive(Debug, ValueEnum, Clone, Copy, Serialize)]
pub enum Location {
    Auckland,
    BayOfPlenty,
    Christchurch,
    Invercargill,
    NewPlymouth,
    Northland,
    Mahia,
    Otago,
    Wellington,
    Westland,
}

/// These strings match up with the URL segment that is used to retrieve the radar image for the
/// location, and are used to generate the URL to retrieve an image.
///
/// (Note that there is no `serde` rename option that supports pascal-style kebab case as needed
/// here, otherwise using an attribute on the enum would probably be better practice).
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::BayOfPlenty => write!(f, "Bay-of-Plenty"),
            Location::NewPlymouth => write!(f, "New-Plymouth"),
            _ => write!(f, "{self:?}"),
        }
    }
}
