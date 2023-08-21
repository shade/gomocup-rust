
use std::{default, io::BufRead};

use enum_dispatch::enum_dispatch;
use strum::IntoStaticStr;

use crate::{Brain, board::{GameBoard, BoardError}, errors::GomocupError};

use self::{game_context::GameContext, start_command::StartCommand, turn_command::TurnCommand, info_command::InfoCommand, begin_command::BeginCommand, board_command::BoardCommand, end_command::EndCommand};
pub mod game_context;

pub mod input_options;

// Actual command implementations
pub mod start_command;
pub mod turn_command;
pub mod info_command;
pub mod begin_command;
pub mod board_command;
pub mod end_command;

#[derive(Debug, Display)]
pub enum CommandError {
    InvalidCommand,
    InvalidArguments(String),
    BoardError(BoardError),

    IllegalState(String)
}

impl From<BoardError> for CommandError {
    fn from(value: BoardError) -> Self {
        CommandError::BoardError(value)
    }
}

#[derive(Debug)]
pub enum CommandResult {
    Output(String),
    Ok,
    Continue,
    Quit,
}

#[enum_dispatch]
pub trait ExecutableCommand : Default {
    fn execute<G: GameBoard, B: Brain>(&self, brain: &mut B, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError>;
}

#[enum_dispatch]
pub trait ExecutableCommandWithInput {
    fn execute<G: GameBoard, B: Brain, R: BufRead>(&self, input: &mut R, brain: &mut B, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError>;
}

impl<T: ExecutableCommand> ExecutableCommandWithInput for T {
    fn execute<G: GameBoard, B: Brain, R: BufRead>(&self, _: &mut R, brain: &mut B, context: &mut GameContext<G>, args: Vec<String>) -> Result<CommandResult, CommandError> {
        self.execute(brain, context, args)
    }
}

#[derive(IntoStaticStr, Display, EnumString)]
#[enum_dispatch(ExecutableCommandWithInput)]
pub enum Command {
    #[strum(serialize="START")]
    Start(StartCommand),
    #[strum(serialize="TURN")]
    Turn(TurnCommand),
    #[strum(serialize="INFO")]
    Info(InfoCommand),
    #[strum(serialize="BEGIN")]
    Begin(BeginCommand),
    #[strum(serialize="BOARD")]
    Board(BoardCommand),
    #[strum(serialize="END")]
    End(EndCommand)
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