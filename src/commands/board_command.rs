use crate::assert_argument_count;

use super::ExecutableCommand;
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BoardCommand;

impl ExecutableCommand for BoardCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<(), CommandError> {
        assert_argument_count!(args, 2);
        Ok(())
    }
}
