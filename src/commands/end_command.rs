use crate::{assert_argument_count, GameBoard, Brain};

use super::{ExecutableCommand, CommandResult};
use crate::commands::game_context::GameContext;
use crate::commands::CommandError;

#[derive(Default)]
pub struct EndCommand;

impl ExecutableCommand for EndCommand {
    fn execute<G: GameBoard, B: Brain>(&self, brain: &mut B, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError> {
        assert_argument_count!(args, 0);
        brain.end();
        Ok(CommandResult::Quit)
    }
}

#[cfg(test)]
mod test {
    use assert_matches::assert_matches;
    use rstest::rstest;

    use crate::{Brain, commands::{game_context::GameContext, CommandError}, board::MockGameBoard, brain::MockBrain};
    use super::*;

    #[rstest]
    #[case(vec!["1", "2"])]
    #[case(vec!["a"])]
    fn test_end_command_requires_no_args(#[case] args: Vec<&str>) {
        let args = args.into_iter().map(|s| s.to_string()).collect();

        let mut brain = MockBrain::default();
        let mut context = GameContext::<MockGameBoard>::default();
        let command = EndCommand::default();

        let result = command.execute(&mut brain, &mut context, args);
        assert_matches!(result, Err(CommandError::InvalidArguments(_)));
    }

    #[test]

    fn test_end_command_calls_end() {
        let mut brain = MockBrain::default();
        brain.expect_end().times(1).return_const(());

        let mut context = GameContext::<MockGameBoard>::default();
        let command = EndCommand::default();

        let result = command.execute(&mut brain, &mut context, vec![]);
        assert_matches!(result, Ok(CommandResult::Quit));
    }
}
