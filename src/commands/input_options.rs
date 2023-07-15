


pub struct InputOptions {
    pub command: String,
    pub args: Vec<String>
}

impl From<String> for InputOptions {
    pub fn from(self) -> InputOptions {
        // Split ...
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
pub mod tests {
    #[test]
    fn 
}