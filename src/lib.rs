mod board;
mod brain;

#[macro_use]
extern crate strum;

use core::panic;
use std::{
    any::Any,
    cell::RefCell,
    io::{Empty, StdinLock},
};

pub use board::GameBoard;
use commands::{game_context::GameContext, ExecutableCommand, CommandResult};
pub use errors::GomocupError;
use strum::Display;
mod commands;
mod errors;

use commands::input_options::InputOptions;
use std::io::{BufRead, Read};

pub use brain::Brain;

pub fn run<T: Brain, G: GameBoard>(brain: T) -> Result<(), GomocupError> {
    let guarded_reader = std::io::stdin().lock();
    run_inner::<StdinLock<'_>, T, G>(guarded_reader, brain)
}

fn run_inner<T: BufRead, V: Brain, G: GameBoard>(mut input: T, brain: V) -> Result<(), GomocupError> {
    let mut context: GameContext<G> = GameContext::default();

    loop {
        let mut buffer = String::new();
        input
            .read_line(&mut buffer)
            .map_err(|err| io_error!(err, "Error reading from stdin"))?;

        let opts = InputOptions::try_from(buffer)?;

        match opts.command.execute(&mut context, opts.args) {
            Ok(cmd) => match cmd {
                CommandResult::Continue => {}
                CommandResult::Output(output) => println!("{}", output),
                CommandResult::Ok => println!("OK"),
                CommandResult::Quit => break Ok(()),
            },
            Err(err) => println!("ERROR {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, Read},
        process::Command,
    };

    use assert_cmd::prelude::*;
    use assert_matches::assert_matches;
    use mockall::mock;

    use crate::{brain::MockBrain, run, run_inner, GameBoard, board::MockGameBoard};

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

        assert_matches!(run_inner::<_,_,MockGameBoard>(mock_reader, brain), Err(_));
    }

    #[test]
    fn test_run_unknown_command() {}

    fn get_command() -> Command {
        Command::cargo_bin("moomoo").unwrap()
    }
}
