use crate::client::ApiClient;
use crate::error::Result;
use crate::path_parser::{parse_path, ApiPath};
use serde_json::Value;

/// Execute a query for the given path
pub fn execute_query(client: &ApiClient, path_str: &str) -> Result<QueryResult> {
    let api_path = parse_path(path_str)?;
    let data = client.get(&api_path.path)?;

    Ok(QueryResult {
        path: api_path,
        data,
    })
}

/// Execute a query for tiles with pagination
pub fn execute_tiles_query(client: &ApiClient, offset: u32, limit: u32) -> Result<QueryResult> {
    let params = [
        ("offset", offset.to_string()),
        ("limit", limit.to_string()),
    ];
    let data = client.get_with_params("tiles", &params)?;

    Ok(QueryResult {
        path: ApiPath {
            path: format!("tiles?offset={}&limit={}", offset, limit),
            endpoint_type: crate::path_parser::EndpointType::Tiles,
        },
        data,
    })
}

#[derive(Debug)]
pub struct QueryResult {
    pub path: ApiPath,
    pub data: Value,
}
