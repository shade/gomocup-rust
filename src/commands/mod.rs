
use std::default;

use enum_dispatch::enum_dispatch;
use strum::IntoStaticStr;

use crate::{Brain, game_board::GameBoard, errors::MooMooError};

use self::{game_context::GameContext, start_command::StartCommand};
pub mod game_context;

pub mod input_options;

// Actual command implementations
pub mod start_command;

#[enum_dispatch]
pub trait ExecutableCommand : Default {
    fn execute(&self, context: &mut GameContext, args: Vec<String>) -> Result<(), MooMooError>;
}

#[derive(IntoStaticStr, Display, EnumString)]
#[enum_dispatch(ExecutableCommand)]
pub enum Command {
    #[strum(serialize="START")]
    Start(StartCommand),
    /*
    #[strum(serialize="TURN")]
    Turn(u32),
    #[strum(serialize="BEGIN")]
    Begin(u32),
    #[strum(serialize="BOARD")]
    Board(u32),
    #[strum(serialize="INFO")]
    Info(u32),
    #[strum(serialize="END")]
    End(u32), */
}


impl Default for Command {
    fn default() -> Self {
        Command::Start(StartCommand::default())
    }
}