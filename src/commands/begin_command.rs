use crate::assert_argument_count;

use super::{ExecutableCommand, CommandResult};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BeginCommand;

impl ExecutableCommand for BeginCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 0);
        Ok(CommandResult::Nop)
    }
}
