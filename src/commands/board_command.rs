use std::io::BufRead;

use crate::{assert_argument_count, Brain, GameBoard};

use super::{CommandResult, ExecutableCommand, ExecutableCommandWithInput};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BoardCommand;

impl ExecutableCommandWithInput for BoardCommand {
    fn execute<G: GameBoard, B: Brain, R: BufRead>(
        &self,
        input: &mut R,
        brain: &mut B,
        context: &mut GameContext<G>,
        args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 0);
        loop {




        }
        // Read in the coordinates..


        // Place the pieces.



        return Ok(CommandResult::Continue);
    }
}
