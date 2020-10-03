use thiserror::Error;

#[derive(Debug, Error)]
pub enum InesError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, InesError>;
