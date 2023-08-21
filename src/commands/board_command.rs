use crate::{assert_argument_count, GameBoard};

use super::{ExecutableCommand, CommandResult};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BoardCommand;

impl ExecutableCommand for BoardCommand {
    fn execute<G: GameBoard>(&self, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 2);
        let row = args.get(0);
        let col = args.get(1);
        
        if let Some(board) = context.board.as_mut() {
            //board.get_piece(row, col);
        } else {

        }


        Ok(CommandResult::Ok)
    }
}
