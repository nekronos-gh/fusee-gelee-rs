use std::fmt;
use std::io;

#[derive(Debug)]
pub enum RcmError {
    Io(io::Error),
    PayloadTooLarge { actual: usize, maximum: usize },
}

impl From<io::Error> for RcmError {
    fn from(error: io::Error) -> Self {
        RcmError::Io(error)
    }
}

impl std::error::Error for RcmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for RcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RcmError::Io(err) => write!(f, "I/O error: {}", err),
            Self::PayloadTooLarge { actual, maximum } => {
                write!(f, "Payload too large: {actual} bytes (maximum: {maximum})")
            }
        }
    }
}
