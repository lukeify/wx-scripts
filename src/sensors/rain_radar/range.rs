use clap::ValueEnum;
use std::fmt;
use serde::Serialize;

#[derive(Debug, ValueEnum, Clone, Copy, Serialize)]
pub enum Range {
    Wide,
    Close,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Range::Close => write!(f, "120K"),
            Range::Wide => write!(f, "300K"),
        }
    }
}

impl std::str::FromStr for Range {
    type Err = ();

    fn from_str(input: &str) -> Result<Range, Self::Err> {
        match input {
            "close" => Ok(Range::Close),
            "wide" => Ok(Range::Wide),
            _ => Err(()),
        }
    }
}
