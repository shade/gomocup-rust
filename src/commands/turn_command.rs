use std::num::ParseIntError;

use crate::{commands::CommandResult, GameBoard, Brain, assert_argument_count, board::GamePiece};

use super::{ExecutableCommand, game_context::GameContext, CommandError};

#[derive(Default)]
pub struct TurnCommand;

impl ExecutableCommand for TurnCommand {
    fn execute<G: GameBoard, B: Brain>(&self, brain: &mut B, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 1);

        let coordinates = args.get(0)
            .ok_or(CommandError::InvalidArguments("Expected 1 argument, got 0".to_string()))?
            .split(",")
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()
            .map_err(|e| CommandError::InvalidArguments(format!("Invalid coordinates: {}", e)))?;

        if coordinates.len() != 2 {
            return Err(CommandError::InvalidArguments(format!("Expected 2 comma seperated coordinates, got {:?}", coordinates)));
        }

        let (row, col) = (coordinates[0], coordinates[1]);

        context.board.as_mut()
            .ok_or(CommandError::IllegalState("Board not initialized".to_string()))?
            .place(row, col, GamePiece::Black)
            .map_err(|e| CommandError::IllegalState(format!("Could not place piece at {}, {}: {:?}", row, col, e)))?;

        Ok(CommandResult::Ok)
    }
}


#[cfg(test)]
mod test {
    use crate::{board::{MockGameBoard, BoardError}, brain::MockBrain};

    use super::*;
    use assert_matches::assert_matches;
    use mockall::{predicate::eq, Any};
    use rstest::rstest;

    #[rstest]
    #[case(vec!["a,a"])]
    #[case(vec!["1,2,3"])]
    #[case(vec!["1"])]
    #[case(vec!["a"])]
    #[case(vec![])]
    #[case(vec!["12934"])]
    fn test_invalid_args_fails(#[case] input: Vec<&str>) {
        let mut brain = MockBrain::new();
        let cmd = TurnCommand::default();
        let args = input.into_iter().map(|s| s.to_string()).collect();

        assert_matches!(cmd.execute::<MockGameBoard, _>(&mut brain, &mut GameContext::default(), args), Err(CommandError::InvalidArguments(_)))
    }

    #[test]
    fn test_calls_place() {
        let mut brain = MockBrain::new();
        let mut context = GameContext::default();
        let mut board = MockGameBoard::default();
        board.expect_place()
            .with(eq(1), eq(2), eq(GamePiece::Black))
            .times(1)
            .returning(|_, _, _| Ok(()));

        context.board = Some(board);

        let cmd = TurnCommand::default();
        let args = vec!["1,2"].into_iter().map(|s| s.to_string()).collect();

        assert_matches!(cmd.execute(&mut brain, &mut context, args), Ok(CommandResult::Ok));
    }

    #[test]
    fn test_bubbles_up_board_error() {
        let mut brain = MockBrain::new();
        let mut context = GameContext::default();
        let mut board = MockGameBoard::default();
        board.expect_place()
            .with(eq(1), eq(2), eq(GamePiece::Black))
            .times(1)
            .returning(|_, _, _| Err(BoardError::InvalidPlace("".to_string())));

        context.board = Some(board);

        let cmd = TurnCommand::default();
        let args = vec!["1,2"].into_iter().map(|s| s.to_string()).collect();

        assert_matches!(cmd.execute(&mut brain, &mut context, args), Err(CommandError::IllegalState(_)));
    }
}