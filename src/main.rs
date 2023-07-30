mod game_board;
mod brain;

#[macro_use]
extern crate strum;

use core::panic;
use std::{cell::RefCell, any::Any, io::{StdinLock, Empty}};

pub use brain::Brain;
pub use game_board::GameBoard;
use brain::{example_brains::random::RandomBrain};
use commands::{game_context::GameContext, ExecutableCommand};
use commands::Command;
use errors::MooMooError;
use game_board::empty::EmptyBoard;
use strum::Display;
mod commands;
mod errors;

use commands::input_options::InputOptions;
use std::io::{BufRead, Read};

fn main() -> Result<(), MooMooError> {

    let guarded_reader = std::io::stdin().lock();
    run::<StdinLock<'_>, RandomBrain>(guarded_reader, RandomBrain::new())
}

fn run<T: BufRead, V: Brain>(mut input: T, brain: V) -> Result<(), MooMooError> {
    let mut context = GameContext::default();

    //brain;

    loop {
        let mut buffer = String::new();
        input
            .read_line(&mut buffer)
            .map_err(|err| io_error!(err, "Error reading from stdin"))?;

        let opts = InputOptions::try_from(buffer)?;

        opts.command.execute(&mut context, opts.args)?;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_bad_input() {}

    #[test]
    fn test_run_unknown_command() {}
}
