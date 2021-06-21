//! Custom error library for Kaleidoscope.

use std::{
    error,
    fmt::{Debug, Display, self}
};

/// The traits every ErrorKind enum must satisfy.
/// If your enum implements all of the following traits,
/// then [`crate::ErrorKind`] gets automatically implemented.
/// 
/// 1. [`Clone`],
/// 2. [`Debug`],
/// 3. [`Display`],
/// 4. [`Eq`]
pub trait ErrorKind: Clone + Debug + Display + Eq {}

impl<T: Clone + Debug + Display + Eq> ErrorKind for T {}

/// A struct representing an error.
#[derive(Debug)]
pub struct Error<EK: ErrorKind> {
    description: String,
    errorkind: EK,
    source: Option<Box<dyn error::Error + 'static>>
}

impl<EK: ErrorKind> Error<EK> {
    /// A new error.
    pub fn new(
        description: &dyn AsRef<str>,
        errorkind: EK,
        source: Option<Box<dyn error::Error>>
    ) -> Self {
        Self {
            description: description.as_ref().to_string(),
            errorkind,
            source
        }
    }

    /// A new error from with another error as the source.
    pub fn from_err(err: Box<dyn error::Error>, errorkind: EK) -> Self {
        Self {
            description: format!("{}", err),
            errorkind,
            source: Some(err)
        }
    }
}

impl<EK: ErrorKind> Clone for Error<EK> {
    fn clone(&self) -> Self {
        Self::new(&self.description, self.errorkind.clone(), None)
    }
}

impl<EK: ErrorKind> Display for Error<EK> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.errorkind, self.description)
    }
}

impl<EK: ErrorKind> error::Error for Error<EK> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            None => None,
            Some(source) => Some(&**source)
        }
    }
}

/// A special [`std::result::Result`] type for Kaleidoscope.
/// Instead of an error type parameter, you are instead asked for an
/// ErrorKind enum type which implements the traits specified by
/// [`crate::ErrorKind`]. This error kind enum is used by
/// [`crate::Error`] to classify the error that has occurred.
pub type Result<T, EK> = std::result::Result<T, Error<EK>>;
