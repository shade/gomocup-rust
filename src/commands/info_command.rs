use crate::{assert_argument_count, commands::ExecutableCommand, errors::GomocupError, GameBoard};

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
    Folder,
}

#[derive(Default)]
pub struct InfoCommand;

impl ExecutableCommand for InfoCommand {
    fn execute<G: GameBoard> (
        &self,
        context: &mut GameContext<G>,
        mut args: Vec<String>,
    ) -> Result<CommandResult, CommandError> {
        assert_argument_count!(&args, 2);

        let raw_key = args.remove(0);

        let key: InfoKey = raw_key
            .parse()
            .map_err(|e| CommandError::InvalidArguments(format!("Unsupported INFO key: {}, error={}", raw_key, e)))?;

        let value = args.remove(0);

        Ok(CommandResult::Ok)
    }
}
