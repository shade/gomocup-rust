mod brain;

#[macro_use]
extern crate strum;

use core::panic;
use std::{cell::RefCell, error::Error, str::SplitInclusive};

use strum::Display;
mod brains;

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
    

    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|err| read_io_error!(err, "Error reading from stdin"))?;

        let mut spliterator = buffer.trim().split_inclusive(" ");
        let command = spliterator.next().ok_or(read_error!(
            "Could not extract command, no initial space found"
        ))?;

        
        match command {
            "START" => {
                let args = extract_arguments(spliterator, Some(1))?;
                println!("{:?} with args {:?}", command, args);
            }
            "TURN" => {
                let args = extract_arguments(spliterator, Some(2))?;
                println!("{:?} with args {:?}", command, args);
            }
            "BEGIN" => {
                let args = extract_arguments(spliterator, Some(0))?;
                println!("{:?} with args {:?}", command, args);
            }
            "BOARD" => {
                let args = extract_arguments(spliterator, Some(0))?; 
            }
            "INFO" => {
                let args = extract_arguments(spliterator, None)?;
                println!("{:?} with args {:?}", command, args);
            }
            "END" => {
                let args = extract_arguments(spliterator, Some(0))?;
                println!("{:?} with args {:?}", command, args);
            }
            _ => {
                panic!("Unknown command {}", command);
            }
        }
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
