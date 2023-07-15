mod brain;

#[macro_use]
extern crate strum;

use core::panic;
use std::cell::RefCell;

use brain::Brain;
use commands::Command;
use commands::context::Context;
use strum::Display;
mod brain;
mod commands;

use std::io::{Read, BufRead};

#[derive(Display, Debug)]
enum MooMooError {
    ReadIOError(std::io::Error, String),
    ReadError(String),
}

macro_rules! read_io_error {
    ($err: tt, $($arg:tt)*) => {{
        MooMooError::ReadIOError($err, format!($($arg)*))
    }};
}

macro_rules! read_error {
    ($($arg:tt)*) => {{
        MooMooError::ReadError(format!($($arg)*))
    }};
}

fn main() -> Result<(), MooMooError> {
    let mut brain: RefCell<Option<dyn Brain>> = RefCell::new(None);
    let guarded_reader = std::io::stdin().lock();
    run(guarded_reader, brain)
}

fn run<T: BufRead, U, V: Brain<U>>(mut input: T, brain: V) -> Result<(), MooMooError>  {
    let mut context = Context::new();

    loop {
        let mut buffer = String::new();
        input
            .read_line(&mut buffer)
            .map_err(|err| read_io_error!(err, "Error reading from stdin"))?;

        let mut spliterator = buffer.trim().split_inclusive(" ");
        let command = spliterator.next().ok_or(read_error!(
            "Could not extract command, no initial space found"
        ))?;

        let command: Command = command
            .parse()
            .map_err(|err| {
                match err {
                    strum::ParseError::VariantNotFound => read_error!("Unimplemented command {}", command),
                    err @ _ => read_error!("Unexpected error parsing command: {:?}", err)
                }
            })?;

        command.execute(&mut context);
    }
}

fn extract_arguments<'a, T: Iterator<Item = &'a str>>(
    argument_str: T,
    expected_count: Option<usize>,
) -> Result<Vec<&'a str>, MooMooError> {
    let arguments: Vec<&str> = argument_str.collect::<Vec<&str>>();

    if let Some(expected_count) = expected_count {
        if arguments.len() != expected_count {
            return Err(read_error!(
                "Expected {} arguments, got {}",
                expected_count,
                arguments.len()
            ));
        }
    }

    Ok(arguments)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_bad_input() {
        
    }

    #[test]
    fn test_run_unknown_command() {

    }
}