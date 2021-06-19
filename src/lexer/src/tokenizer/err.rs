use std::{error, fmt};

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    FileIOError,
    Other
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Error {
    description: String,
    errorkind: ErrorKind
}

impl Error {
    pub fn new(description: String, errorkind: ErrorKind) -> Self {
        Self {description, errorkind}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
