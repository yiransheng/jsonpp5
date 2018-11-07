use std::fmt;
use std::io::Error as IoError;

use json5::Error as ParseError;

#[derive(Debug)]
pub enum Error {
    IoFail(IoError),
    ParseFail(ParseError),
    NoInput,
}
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoFail(ref err) => err.description(),
            Error::ParseFail(ref err) => err.description(),
            Error::NoInput => "Missing input (use --stdin maybe?)",
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoFail(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseFail(err)
    }
}
