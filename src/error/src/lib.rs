//! Custom error library for Kaleidoscope.

use std::{
    error,
    fmt::{Debug, Display, self}
};

/// A struct representing an error.
#[derive(Debug)]
pub struct Error<EK: Clone + Debug + Display> {
    description: String,
    errorkind: EK,
    source: Option<Box<dyn error::Error + 'static>>
}

impl<EK: Clone + Debug + Display> Error<EK> {
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

impl<EK: Clone + Debug + Display> Clone for Error<EK> {
    fn clone(&self) -> Self {
        Self::new(&self.description, self.errorkind.clone(), None)
    }
}

impl<EK: Clone + Debug + Display> Display for Error<EK> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.errorkind, self.description)
    }
}

impl<EK: Clone + Debug + Display> error::Error for Error<EK> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            None => None,
            Some(source) => Some(&**source)
        }
    }
}

pub type Result<T, EK> = std::result::Result<T, Error<EK>>;
