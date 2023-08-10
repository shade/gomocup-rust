mod brain;
mod game_board;

#[macro_use]
extern crate strum;

use core::panic;
use std::{
    any::Any,
    cell::RefCell,
    io::{Empty, StdinLock},
};


use commands::{game_context::GameContext, ExecutableCommand};
use errors::GomocupError;
use game_board::empty::EmptyBoard;
pub use game_board::GameBoard;
use strum::Display;
mod commands;
mod errors;

use commands::input_options::InputOptions;
use std::io::{BufRead, Read};

pub use brain::Brain;

pub fn run<T: Brain>(brain: T) -> Result<(), GomocupError> {
    let guarded_reader = std::io::stdin().lock();
    run_inner::<StdinLock<'_>, T>(guarded_reader, brain)
}

fn run_inner<T: BufRead, V: Brain>(mut input: T, brain: V) -> Result<(), GomocupError> {
    let mut context = GameContext::default();

    loop {
        let mut buffer = String::new();
        input
            .read_line(&mut buffer)
            .map_err(|err| io_error!(err, "Error reading from stdin"))?;

        let opts = InputOptions::try_from(buffer)?;

        match opts.command.execute(&mut context, opts.args)? {
            commands::CommandResult::Nop => {}
            commands::CommandResult::Quit => break Ok(()),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, Read},
        process::Command,
    };

    use assert_cmd::prelude::*;
    use assert_matches::assert_matches;
    use mockall::mock;

    use crate::{brain::MockBrain, run};

    mock! {
        Reader{}
        impl Read for Reader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
        }
        impl BufRead for Reader {
            fn fill_buf(&mut self) -> std::io::Result<&'static [u8]>;

            fn consume(&mut self, amt: usize);
        }
    }

    #[test]
    fn test_run_bad_input() {
        let brain = MockBrain::new();
        let mut mock_reader = MockReader::default();

        mock_reader
            .expect_fill_buf()
            .returning(|| Err(std::io::Error::new(std::io::ErrorKind::Other, "test error")));

        assert_matches!(run(mock_reader, brain), Err(_));
    }

    #[test]
    fn test_run_unknown_command() {}

    fn get_command() -> Command {
        Command::cargo_bin("moomoo").unwrap()
    }
}
 */