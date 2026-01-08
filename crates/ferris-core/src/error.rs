use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Unsupported(&'static str),
    Io(std::io::Error),
}

impl Error {
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "{msg}"),
            Error::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            Error::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
