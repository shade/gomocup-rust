use std::{sync::Arc, time::Duration};

use crate::{
    assert_argument_count, commands::ExecutableCommand, config, errors::GomocupError, Brain,
    GameBoard, board::MockGameBoard,
};

use super::{game_context::GameContext, CommandError, CommandResult};

#[derive(IntoStaticStr, Display, EnumString)]
pub enum InfoKey {
    #[strum(serialize = "timeout_turn")]
    TimeoutTurn,
    #[strum(serialize = "timeout_match")]
    TimeoutMatch,
    #[strum(serialize = "max_memory")]
    MaxMemory,
    #[strum(serialize = "time_left")]
    TimeLeft,
    #[strum(serialize = "game_type")]
    GameType,
    #[strum(serialize = "rule")]
    Rule,
    #[strum(serialize = "evaluate")]
    Evaluate,
    #[strum(serialize = "folder")]
    PersistedFilesFolder,
}

#[derive(Default)]
pub struct InfoCommand;

impl ExecutableCommand for InfoCommand {
    fn execute<G: GameBoard, B: Brain>(
        &self,
        brain: &mut B,
        context: &mut GameContext<G>,
        mut args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        assert_argument_count!(&args, 2);

        let raw_key = args.remove(0);

        let key: InfoKey = raw_key.parse().map_err(|e| {
            CommandError::InvalidArguments(format!(
                "Unsupported INFO key: {}, error={}",
                raw_key, e
            ))
        })?;

        let value = args.remove(0);

        match key {
            InfoKey::TimeoutTurn => {
                let timeout_turn = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid timeout_turn value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.timeout_turn = Duration::from_millis(timeout_turn);
            }
            InfoKey::TimeoutMatch => {
                let timeout_match = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid timeout_match value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.timeout_match = Duration::from_millis(timeout_match);
            }
            InfoKey::MaxMemory => {
                let max_memory = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid max_memory value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.max_memory = max_memory;
            }
            InfoKey::TimeLeft => {
                let time_left = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid time_left value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.time_left = Duration::from_millis(time_left);
            }
            InfoKey::GameType => {
                let game_type = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid game_type value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.game_type = game_type.into();
            }
            InfoKey::Rule => {
                let rule = value.parse::<u64>().map_err(|e| {
                    CommandError::InvalidArguments(format!(
                        "Invalid rule value: {}, error={}",
                        value, e
                    ))
                })?;

                context.config.game_rule = rule.into();
            }
            InfoKey::PersistedFilesFolder => {
                context.config.persisted_files_folder = Arc::new(value);
            }
            InfoKey::Evaluate => {
                // Evaluate is a no-op since we're not supporting DEBUG atm.
            }
        }

        brain.set_config(context.config.clone())?;

        Ok(CommandResult::Ok)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::MockGameBoard,
        brain::MockBrain,
        config::{GameRules, GameType}, BrainError,
    };

    use super::*;
    use mockall::{mock, predicate::*};
    use std::time::Duration;

    #[test]
    fn test_info_command_timeout_turn() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["timeout_turn".to_string(), "5000".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.timeout_turn == Duration::from_millis(5000))
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_timeout_match() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["timeout_match".to_string(), "30000".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.timeout_match == Duration::from_millis(30000))
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_max_memory() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["max_memory".to_string(), "1024".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.max_memory == 1024)
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_time_left() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["time_left".to_string(), "60000".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.time_left == Duration::from_millis(60000))
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_game_type() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["game_type".to_string(), "1".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.game_type == GameType::BrainOpponent)
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_rule() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["rule".to_string(), "5".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| config.game_rule == GameRules::from(5))
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_persisted_files_folder() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["folder".to_string(), "/tmp".to_string()];
        let info_command = InfoCommand;

        mock_brain
            .expect_set_config()
            .withf(|config| *config.persisted_files_folder == "/tmp")
            .times(1)
            .returning(|_| Ok(()));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        assert!(result.is_ok());
    }

    #[test]
    fn test_info_command_propagate_error() {
        let mut mock_brain = MockBrain::new();
        let mut context = GameContext::<MockGameBoard>::default();
        let args = vec!["timeout_turn".to_string(), "5000".to_string()];
        let info_command = InfoCommand;

        // Expectation: The set_config method should be called on the mock Brain
        // We are simulating an error being returned from the Brain
        mock_brain.expect_set_config()
            .times(1)
            .returning(|_| Err(BrainError::CommonError("Test error".to_string())));

        let result = info_command.execute(&mut mock_brain, &mut context, args);

        // Expectation: The command should return an error and propagate the BrainError
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::BrainError(_)));
    }
}
