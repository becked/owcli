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

impl OwcliError {
    pub fn from_status(status: reqwest::StatusCode, body: &str) -> Self {
        match status.as_u16() {
            503 => OwcliError::GameUnavailable,
            404 => OwcliError::NotFound(body.to_string()),
            code => OwcliError::Api {
                message: body.to_string(),
                code: Some(code),
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, OwcliError>;
