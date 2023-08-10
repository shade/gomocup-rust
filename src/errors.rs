use crate::commands::CommandError;


#[derive(Display, Debug)]
pub enum GomocupError {
    IOError(std::io::Error, String),
    ReadError(String),
    CommandError(CommandError)
}

impl From<CommandError> for GomocupError {
    fn from(err: CommandError) -> Self {
        GomocupError::CommandError(err)
    }
}

#[macro_export]
macro_rules! io_error {
    ($err: tt, $($arg:tt)*) => {{
        GomocupError::IOError($err, format!($($arg)*))
    }};
}
#[macro_export]
macro_rules! read_error {
    ($($arg:tt)*) => {{
        GomocupError::ReadError(format!($($arg)*))
    }};
}
