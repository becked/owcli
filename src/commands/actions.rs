use crate::cli::Action;
use crate::client::{fetch, types::CommandResult, ApiClient};
use crate::error::Result;

/// Execute a game action command
pub async fn execute_action(client: &ApiClient, action: &Action) -> Result<CommandResult> {
    let command = action.to_game_command();
    fetch(client.inner.execute_command(&command)).await
}
