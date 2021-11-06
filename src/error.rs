use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[cfg_attr(feature = "error-serialize", derive(serde::Serialize))]
#[derive(Debug)]
/// Error thrown while processing
/// You can use Custom(C) for your own errors.
pub enum Error<C: Display> {
    PatternAlreadyRegistered,
    TagNameEmpty,
    TagNotEnded,
    TagEndIncorrect,
    TagEndInvalid,
    TagHeaderInvalid,
    UnknownTag,
    TagNotFound,
    ArgNotFound,

    Custom(C),
}

impl<C: Display + Debug> Display for Error<C> {
    fn fmt(&self, buffer: &mut Formatter) -> FmtResult {
        write!(buffer, "{:?}", self)
    }
}

impl<C: std::fmt::Display + std::fmt::Debug> std::error::Error for Error<C> {}

impl<E: Display> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::Custom(e)
    }
}
