use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pelite(#[from] pelite::Error),

    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),

    #[error("{0}")]
    Other(&'static str),
}
