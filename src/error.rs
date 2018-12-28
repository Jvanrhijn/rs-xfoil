use std::{error, fmt};
use std::num;
use std::io;

#[derive(Debug)]
pub enum XfoilError {
    IoError(io::Error),
    ParseError(num::ParseFloatError),
}

impl From<io::Error> for XfoilError {
    fn from(error: io::Error) -> Self {
        XfoilError::IoError(error)
    }
}

impl From<num::ParseFloatError> for XfoilError {
    fn from(error: num::ParseFloatError) -> Self {
        XfoilError::ParseError(error)
    }
}

impl fmt::Display for XfoilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal xfoil error")
    }
}

impl error::Error for XfoilError {
    fn description(&self) -> &str {
        "Error occured in xfoil calculation"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub type Result<T> = std::result::Result<T, XfoilError>;

