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
            .map_err(|err| io_error!(err, "Error reading from stdin"))?;

        let opts = InputOptions::from(buffer)?;
        command.execute(&mut context, opts)?;
    }
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