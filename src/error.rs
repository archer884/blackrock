use bitreel::error::Error as BitreelError;
use reqwest;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Cause = Box<error::Error>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Option<Cause>,
    description: Cow<'static, str>,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// The selected video format was unavailable, or no video formats were available.
    Format,

    /// An error occurred in disk io.
    IO,

    /// An error occurred in network io.
    Network,

    /// The selected video was not found.
    NotFound,
}

impl Error {
    pub fn not_found(error: BitreelError) -> Self {
        Error {
            kind: ErrorKind::NotFound,
            cause: Some(Box::new(error)),
            description: Cow::from("Video not found"),
        }
    }

    pub fn format_unavailable() -> Self {
        Error {
            kind: ErrorKind::Format,
            cause: None,
            description: Cow::from("Requested format unavailable")
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause.as_ref().map(|s| s.as_ref())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: ErrorKind::IO,
            cause: Some(Box::new(error)),
            description: Cow::from("IO failure"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            kind: ErrorKind::Network,
            cause: Some(Box::new(error)),
            description: Cow::from("Network resource unavailable"),
        }
    }
}
