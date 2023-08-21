use crate::GameBoard;

use super::{ExecutableCommand, CommandError, CommandResult, game_context};

#[derive(Default)]
pub struct StartCommand;

impl ExecutableCommand for StartCommand {
    fn execute<G: GameBoard> (&self, context: &mut super::game_context::GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError> {
        if args.len() != 1 {
            return Err(CommandError::InvalidArguments("Expected exactly one argument, board size for 'START'".to_string()))
        }

        let size = args[0].parse::<u64>()
            .map_err(|e| CommandError::InvalidArguments(format!("Invalid board size: {}", e)))?;

        context.board.replace(G::new(size)?);

        Ok(CommandResult::Ok)
    }
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use rstest::rstest;

    use crate::{board::{MockGameBoard, BoardError}, commands::{game_context::{self, GameContext}, ExecutableCommand, CommandResult}};

    use super::StartCommand;

    #[rstest]
    #[case(vec!["1", "2", "3"])]
    #[case(vec!["1", "2"])]
    #[case(vec!["a", "b"])]
    #[case(vec!["a"])]
    #[case(vec![])]
    fn test_start_command_rejects_invalid_args(#[case] args: Vec<&str>) {
        let args = args.into_iter().map(|s| s.to_string()).collect();
        let mut game_context = GameContext::<MockGameBoard>::default();

        assert_matches!(StartCommand::default()
        .execute(&mut game_context, args), Err(crate::commands::CommandError::InvalidArguments(_)));

    }

    #[test]
    fn test_start_command() {
        let board_size = 12;
        let mut game_context = GameContext::<MockGameBoard>::default();
        let context = MockGameBoard::new_context();
        context.expect()
            .returning(move |value| {
                assert_eq!(value, board_size);
                Ok(MockGameBoard::default())
            });

        let result = StartCommand::default()
            .execute(&mut game_context, vec![format!("{}", board_size)]);


        assert_matches!(result, Ok(CommandResult::Ok));
        assert!(game_context.board.is_some());

        drop(context);
    }
}