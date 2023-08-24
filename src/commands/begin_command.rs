use crate::{assert_argument_count, board, Brain, GameBoard};

use super::{CommandResult, ExecutableCommand};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct BeginCommand;

impl ExecutableCommand for BeginCommand {
    fn execute<G: GameBoard + 'static, B: Brain>(
        &self,
        brain: &mut B,
        context: &mut GameContext<G>,
        args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 0);
        if let Some(board) = context.board.as_mut() {
            Ok(brain
                .next_move(board)
                .map(|(row, col)| CommandResult::Output(format!("{},{}", row, col)))?)
        } else {
            Err(CommandError::IllegalState(
                "No board found, did you call START?".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use std::ptr;

    use super::BeginCommand;
    use crate::{
        board::MockGameBoard,
        brain::MockBrain,
        commands::{
            game_context::GameContext, CommandError, ExecutableCommand, CommandResult,
        }, GameBoard, BrainError,
    };
    use assert_matches::assert_matches;
    use mockall::predicate::eq;
    use quickcheck::quickcheck;
    use rand::rngs::mock;

    quickcheck! {
        fn prop_args_must_be_empty(args: Vec<String>) -> bool {
            if args.is_empty() {
                return true;
            }

            let mut brain = MockBrain::default();
            let mut context = GameContext::<MockGameBoard>::default();
            let command = BeginCommand::default();

            let result = command.execute(&mut brain, &mut context, args);
            matches!(result, Err(CommandError::InvalidArguments(_)))
        }
    }

    #[test]
    fn begin_command_emits_brain_move() {
        let mut brain = MockBrain::default();
        let mut mock_board = MockGameBoard::default();
        brain.expect_next_move()
            .times(1).return_once(|_:&mut MockGameBoard| Ok((3, 4)));
        
        
        let mut context = GameContext::default();
        context.board = Some(mock_board);
        let command = BeginCommand::default();

        let result = command.execute(&mut brain, &mut context, vec![]).unwrap();
        assert_matches!(CommandResult::Output(String::from("3,4")), result)
    }

    #[test]
    fn begin_command_emits_no_board_err() {
        let mut brain = MockBrain::default();
        
        let mut context = GameContext::<MockGameBoard>::default();
        let command = BeginCommand::default();

        let result = command.execute(&mut brain, &mut context, vec![]).unwrap_err();
        assert_matches!(result, CommandError::IllegalState(_));
    }

    #[test]
    fn begin_command_brain_error_bubbles_up() {

        let mut brain = MockBrain::default();
        let mock_board = MockGameBoard::default();
        brain.expect_next_move()
            .times(1)
            .return_once(|_:&mut MockGameBoard| Err(BrainError::NoMoveFound));
        
        
        let mut context = GameContext::default();
        context.board = Some(mock_board);
        let command = BeginCommand::default();

        let brain_err = command.execute(&mut brain, &mut context, vec![]).unwrap_err();
        assert_matches!(brain_err, CommandError::BrainError(BrainError::NoMoveFound));
    }
}
