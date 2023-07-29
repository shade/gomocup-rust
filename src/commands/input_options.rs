use crate::{read_error, errors::MooMooError};




pub struct InputOptions {
    pub command: String,
    pub args: Vec<String>
}

impl TryFrom<String> for InputOptions {
    type Error = MooMooError;
    fn try_from(s: String) -> Result<InputOptions, MooMooError> {
        Ok(InputOptions{
            command: "".to_string(),
            args: vec![]
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
