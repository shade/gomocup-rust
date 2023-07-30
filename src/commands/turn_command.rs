use crate::{errors::MooMooError, commands::CommandResult};

use super::{ExecutableCommand, game_context::GameContext, CommandError};

#[derive(Default)]
pub struct TurnCommand;

impl ExecutableCommand for TurnCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<CommandResult, CommandError> {
        println!("Turn command executed");
        Ok(CommandResult::Nop)
    }
}
