//! Error type and other helpers using the [`error-chain`] crate.
//!
//! [`error-chain`]: https://docs.rs/error-chain/

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid scheme: {0:?}")]
    InvalidScheme(String),
    #[error("missing name")]
    MissingName,
    #[error("missing scheme")]
    MissingScheme,
    #[error("missing type")]
    MissingType,
    #[error("utf-8 decoding failed")]
    DecodingError(#[source] std::str::Utf8Error)
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::DecodingError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
