use std::collections::HashMap;
use crate::{commands::{ExecutableCommand, CommandError, CommandResult}, Brain, GameBoard};

use super::game_context::GameContext;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct AboutCommand;

impl ExecutableCommand for AboutCommand {
    fn execute<G: GameBoard, B: Brain>(
        &self,
        brain: &mut B,
        context: &mut GameContext<G>,
        _args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        let mut about_info = brain.about();

        // Add info about this interface
        about_info.insert("interface".to_string(), format!("gomocup_rust v{}", VERSION));

        let mut formatted_info = String::new();
        for (key, value) in about_info.iter() {
            formatted_info.push_str(&format!("{}=\"{}\", ", key, value));
        }

        // Remove trailing comma and space
        if !formatted_info.is_empty() {
            formatted_info.pop();
            formatted_info.pop();
        }

        Ok(CommandResult::Output(formatted_info))
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::{commands::game_context::GameContext, board::MockGameBoard, brain::MockBrain};

    use super::*;
    #[test]
    fn test_about_command_execute() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec![];
        let about_command = AboutCommand::default();

        let mut about_info = HashMap::new();
        about_info.insert("name".to_string(), "SomeBrain".to_string());
        about_info.insert("version".to_string(), "1.0".to_string());

        mock_brain.expect_about()
            .times(1)
            .return_const(about_info);

        let result = about_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
        match result {
            Ok(CommandResult::Output(about_output)) => {
                assert_eq!(about_output.split(", ").count(), 3);
            },
            _ => panic!("Unexpected result"),
        }
    }
}

