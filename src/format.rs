use error::*;
use std::str;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Format {
    Small,
    Medium,
    HD720,
}

impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        match *self {
            Format::Small => "small",
            Format::Medium => "medium",
            Format::HD720 => "hd720",
        }
    }
}

impl str::FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Format> {
        match &*s.to_lowercase() {
            "small" => Ok(Format::Small),
            "medium" => Ok(Format::Medium),
            "hd720" => Ok(Format::HD720),

            _ => Err(Error::format_unsupported()),
        }
    }
}
