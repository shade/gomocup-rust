
#[derive(Display, Debug)]
pub enum MooMooError {
    IOError(std::io::Error, String),
    ReadError(String),
}

#[macro_export]
macro_rules! io_error {
    ($err: tt, $($arg:tt)*) => {{
        MooMooError::IOError($err, format!($($arg)*))
    }};
}
#[macro_export]
macro_rules! read_error {
    ($($arg:tt)*) => {{
        MooMooError::ReadError(format!($($arg)*))
    }};
}
