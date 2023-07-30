use crate::{errors::MooMooError, read_error};

use super::Command;

pub struct InputOptions {
    pub command: Command,
    pub args: Vec<String>,
}

impl TryFrom<String> for InputOptions {
    type Error = MooMooError;
    fn try_from(s: String) -> Result<InputOptions, MooMooError> {
        let mut chunks: Vec<String> = s.split_whitespace().map(String::from).collect();

        if chunks.is_empty() {
            return Err(read_error!("No command provided"));
        }

        let command_str = chunks
        .remove(0);
        let command = command_str
            .parse()
            .map_err(|e| read_error!("Command '{}' not parsable, debug_info={:?}", command_str, e))?;

        Ok(InputOptions {
            command,
            args: chunks,
        })
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
