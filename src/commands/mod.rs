
use std::default;

use enum_dispatch::enum_dispatch;
use strum::IntoStaticStr;

use crate::{Brain, game_board::GameBoard, errors::MooMooError};

use self::{game_context::GameContext, start_command::StartCommand, turn_command::TurnCommand, info_command::InfoCommand};
pub mod game_context;

pub mod input_options;

// Actual command implementations
pub mod start_command;
pub mod turn_command;
pub mod info_command;

#[derive(Debug, Display)]
pub enum CommandError {
    InvalidCommand,
    InvalidArguments(String),
}

#[enum_dispatch]
pub trait ExecutableCommand : Default {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<(), CommandError>;
}

#[derive(IntoStaticStr, Display, EnumString)]
#[enum_dispatch(ExecutableCommand)]
pub enum Command {
    #[strum(serialize="START")]
    Start(StartCommand),
    #[strum(serialize="TURN")]
    Turn(TurnCommand),
    #[strum(serialize="INFO")]
    Info(InfoCommand),
    /*
    #[strum(serialize="BEGIN")]
    Begin(u32),
    #[strum(serialize="BOARD")]
    Board(u32),
    #[strum(serialize="END")]
    End(u32), */
}


impl Default for Command {
    fn default() -> Self {
        Command::Start(StartCommand::default())
    }
}

#[macro_export]
macro_rules! assert_argument_count {
    ($args:expr, $count:literal) => {
        if $args.len() != $count {
            return Err(CommandError::InvalidArguments(format!("Expected {} arguments, got {}", $count, $args.len())));
        }
    };
}