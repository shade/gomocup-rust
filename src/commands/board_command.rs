use crate::assert_argument_count;

use super::{ExecutableCommand, CommandResult};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BoardCommand;

impl ExecutableCommand for BoardCommand {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 2);
        let row = args.get(0);
        let col = args.get(1);
        
        if let Some(board) = context.board {
            board.get_piece(row, col);
        } else {

        }


        Ok(CommandResult::Nop)
    }
}
