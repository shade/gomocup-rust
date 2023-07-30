use crate::assert_argument_count;

use super::ExecutableCommand;
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct EndCommand;

impl ExecutableCommand for EndCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<(), CommandError> {
        assert_argument_count!(args, 0);
        Ok(())
    }
}
