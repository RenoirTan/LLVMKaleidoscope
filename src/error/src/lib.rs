//! Custom error library for Kaleidoscope.

use std::{
    error,
    fmt::{self, Debug, Display}
};

/// The traits every ErrorKind enum must satisfy.
/// If your enum implements all of the following traits,
/// then [`ErrorKind`] gets automatically implemented.
///
/// 1. [`Clone`],
/// 2. [`Debug`],
/// 3. [`Display`],
/// 4. [`Eq`]
pub trait ErrorKind: Clone + Debug + Display + Eq {}

impl<T: Clone + Debug + Display + Eq> ErrorKind for T {}

/// A struct representing an error.
/// As you can see from the type signature, you must provide a type
/// (preferably an enum) that implements the 4 traits listed under the
/// documentation for [`ErrorKind`]. This `EK` is used to classify the type
/// of error that has occurred, and will be shown in a formatted string
/// created by calling [`format!`] and related macros.
#[derive(Debug)]
pub struct Error<EK: ErrorKind> {
    description: String,
    errorkind:   EK,
    source:      Option<Box<dyn error::Error + 'static>>
}

impl<EK: ErrorKind> Error<EK> {
    /// A new error. You can pass in a `description` describing what happened
    /// to trigger the error, the corresponding `errorkind` which classifies
    /// the type of error that occurred and an optional `source` error used
    /// for backtracing.
    pub fn new(description: String, errorkind: EK, source: Option<Box<dyn error::Error>>) -> Self {
        Self {
            description,
            errorkind,
            source
        }
    }

    /// A new error with another error as the source.
    pub fn from_err(err: Box<dyn error::Error>, errorkind: EK) -> Self {
        Self {
            description: format!("{}", err),
            errorkind,
            source: Some(err)
        }
    }

    /// Create a function which converts a source error to an error of type
    /// `Error<EK>`. You must supply a predetermined [`ErrorKind`] to map the
    /// source error to. You can use this function in
    /// [`std::result::Result::map_err`].
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::{stdout, Write};
    ///
    /// use kaleidoscope_error::Error;
    /// use kaleidoscope_macro::impl_display;
    ///
    /// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    /// enum ErrorKind {
    ///     FlushingError,
    ///     Other
    /// }
    /// impl_display!(ErrorKind);
    ///
    /// print!("No new line after this");
    /// let _ = stdout()
    ///     .flush()
    ///     .map_err(Error::factory(ErrorKind::FlushingError));
    /// ```
    pub fn factory<E>(error_kind: EK) -> impl Fn(E) -> Self
    where
        E: error::Error + Sized + 'static
    {
        move |e| Self::from_err(Box::new(e), error_kind.clone())
    }

    /// Convert a source error wrapped in a [`Box`] to an error of type
    /// `Error<EK>`. See [`Error::factory`] for implementation details.
    pub fn boxed_factory(error_kind: EK) -> impl Fn(Box<dyn error::Error + 'static>) -> Self {
        move |e| Self::from_err(e, error_kind.clone())
    }
}

impl<EK: ErrorKind> Clone for Error<EK> {
    fn clone(&self) -> Self {
        Self::new(self.description.clone(), self.errorkind.clone(), None)
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
/// [`ErrorKind`]. This error kind enum is used by
/// [`Error`] to classify the error that has occurred.
pub type Result<T, EK> = std::result::Result<T, Error<EK>>;
