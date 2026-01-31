use crate::config::Config;
use crate::error::{OwcliError, Result};
use std::future::Future;

#[allow(dead_code)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub use generated::types;

pub struct ApiClient {
    pub inner: generated::Client,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let base_url = config.base_url();
        let inner = generated::Client::new_with_client(&base_url, http_client);

        Ok(Self { inner })
    }
}

/// Fetch helper - awaits a progenitor call, extracts response and maps errors
pub async fn fetch<T, E: std::fmt::Debug>(
    future: impl Future<Output = std::result::Result<progenitor_client::ResponseValue<T>, progenitor_client::Error<E>>>,
) -> Result<T> {
    future
        .await
        .map(|r| r.into_inner())
        .map_err(map_progenitor_error)
}

fn map_progenitor_error<T: std::fmt::Debug>(err: progenitor_client::Error<T>) -> OwcliError {
    match &err {
        progenitor_client::Error::ErrorResponse(resp) => {
            let status = resp.status();
            match status.as_u16() {
                503 => OwcliError::GameUnavailable,
                404 => OwcliError::NotFound(format!("{:?}", err)),
                code => OwcliError::Api {
                    message: format!("{:?}", err),
                    code: Some(code),
                },
            }
        }
        progenitor_client::Error::CommunicationError(_) => {
            OwcliError::Other(format!("Communication error: {:?}", err))
        }
        _ => OwcliError::Other(format!("{:?}", err)),
    }
}

/// Helper to check if a command succeeded
pub fn command_succeeded(result: &types::CommandResult) -> bool {
    result.success.unwrap_or(true)
}
