use crate::cli::Action;
use crate::client::{types::CommandResult, ApiClient};
use crate::error::Result;

/// Execute a game action command
pub async fn execute_action(client: &ApiClient, action: &Action) -> Result<CommandResult> {
    let command = action.to_game_command();
    client.execute_command(&command).await
}
