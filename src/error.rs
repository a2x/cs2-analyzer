use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Pelite error: {0}")]
    Pelite(#[from] pelite::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("{0}")]
    Other(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
