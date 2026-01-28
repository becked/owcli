use crate::config::Config;
use crate::error::{OwcliError, Result};
use reqwest::blocking::Client;
use serde_json::Value;
use uuid::Uuid;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: config.base_url(),
        })
    }

    /// Execute a GET request to the given path
    pub fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let response = self.client.get(&url).send()?;

        let status = response.status();
        let body = response.text()?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(OwcliError::from_status(status, &body))
        }
    }

    /// Execute a GET request with query parameters
    pub fn get_with_params(&self, path: &str, params: &[(&str, String)]) -> Result<Value> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let response = self.client.get(&url).query(params).send()?;

        let status = response.status();
        let body = response.text()?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(OwcliError::from_status(status, &body))
        }
    }

    /// Execute a game command
    pub fn command(&self, action: &str, params: Value) -> Result<CommandResponse> {
        let url = format!("{}/command", self.base_url);
        let request_id = Uuid::new_v4().to_string();

        let body = serde_json::json!({
            "action": action,
            "requestId": request_id,
            "params": params
        });

        let response = self.client.post(&url).json(&body).send()?;

        let status = response.status();
        let response_body = response.text()?;

        if status.is_success() {
            let result: CommandResponse = serde_json::from_str(&response_body)?;
            Ok(result)
        } else {
            Err(OwcliError::from_status(status, &response_body))
        }
    }

    /// Validate a command without executing it
    #[allow(dead_code)]
    pub fn validate(&self, action: &str, params: &[(&str, String)]) -> Result<ValidationResponse> {
        let url = format!("{}/validate", self.base_url);
        let mut query_params: Vec<(&str, String)> = vec![("action", action.to_string())];
        query_params.extend(params.iter().cloned());

        let response = self.client.get(&url).query(&query_params).send()?;

        let status = response.status();
        let body = response.text()?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(OwcliError::from_status(status, &body))
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    pub request_id: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub reason: Option<String>,
}
