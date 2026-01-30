use thiserror::Error;

#[derive(Error, Debug)]
pub enum OwcliError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Readline error: {0}")]
    Readline(#[from] rustyline::error::ReadlineError),

    #[error("API error: {message}")]
    Api { message: String, code: Option<u16> },

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Game not available (is it running?)")]
    GameUnavailable,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, OwcliError>;
