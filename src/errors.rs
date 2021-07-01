//! Error type and other helpers using the [`error-chain`] crate.
//!
//! [`error-chain`]: https://docs.rs/error-chain/

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid scheme: {0:?}")]
    InvalidScheme(String),
    #[error("missing name")]
    MissingName,
    #[error("missing scheme")]
    MissingScheme,
    #[error("missing type")]
    MissingType,
}

pub type Result<T> = std::result::Result<T, Error>;
