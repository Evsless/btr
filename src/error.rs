use core::fmt;
use std::io;

#[derive(Debug)]
pub enum BtrError {
    Io(io::Error),
    InvalidData(Option<String>),
    InvalidPeriod(String),
}

#[derive(Debug, PartialEq)]
pub enum BtrErrorKind {
    Io(io::ErrorKind),
    InvalidData,
    InvalidPeriod,
}

impl BtrError {
    pub fn kind(&self) -> BtrErrorKind {
        match self {
            BtrError::Io(e) => BtrErrorKind::Io(e.kind()),
            BtrError::InvalidData(_) => BtrErrorKind::InvalidData,
            BtrError::InvalidPeriod(_) => BtrErrorKind::InvalidPeriod,
        }
    }
}

impl fmt::Display for BtrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BtrError::Io(e) => write!(f, "IO error: {}", e),
            BtrError::InvalidData(None) => write!(f, "Invalid data"),
            BtrError::InvalidData(Some(msg)) => write!(f, "Invalid data: {}", msg),
            BtrError::InvalidPeriod(msg) => write!(f, "Invalid period: {}", msg),
        }
    }
}

impl From<io::Error> for BtrError {
    fn from(err: io::Error) -> Self {
        BtrError::Io(err)
    }
}
