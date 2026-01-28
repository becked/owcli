pub mod table;

use crate::error::Result;
use crate::path_parser::EndpointType;
use serde_json::Value;

/// Format output based on the endpoint type and output mode
pub fn format_output(data: &Value, endpoint_type: &EndpointType, json_mode: bool) -> Result<String> {
    if json_mode {
        Ok(serde_json::to_string_pretty(data)?)
    } else {
        Ok(table::format_table(data, endpoint_type))
    }
}

/// Format a command response
pub fn format_command_response(success: bool, error: Option<&str>, request_id: Option<&str>, json_mode: bool) -> String {
    if json_mode {
        serde_json::json!({
            "success": success,
            "error": error,
            "requestId": request_id
        }).to_string()
    } else if success {
        match request_id {
            Some(id) => format!("Success (request: {})", id),
            None => "Success".to_string(),
        }
    } else {
        match error {
            Some(e) => format!("Error: {}", e),
            None => "Error: Unknown error".to_string(),
        }
    }
}
