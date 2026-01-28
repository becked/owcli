use crate::cli::Action;
use crate::client::{ApiClient, CommandResponse};
use crate::error::Result;

/// Execute a game action command
pub fn execute_action(client: &ApiClient, action: &Action) -> Result<CommandResponse> {
    let (action_name, params) = action.to_api_params();
    client.command(action_name, params)
}
