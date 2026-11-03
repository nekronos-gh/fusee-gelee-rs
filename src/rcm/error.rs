use std::io;
use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum RcmError {
    DeviceNotFound,
    IO(io::Error),
    PayloadNotFound(PathBuf),
    PayloadTooLarge(usize),
    UnsupportedPlatform(String),
}

impl fmt::Display for RcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RcmError::DeviceNotFound => write!(f, "No device in RCM found"),
            RcmError::IO(err) => write!(f, "I/O error: {}", err),
            RcmError::PayloadNotFound(path) => write!(f, "Payload {} not found", path.display()),
            RcmError::PayloadTooLarge(size) => write!(f, "Payload is too large: {} Bytes", size),
            RcmError::UnsupportedPlatform(platform) => {
                write!(
                    f,
                    "Platform \"{}\" is not supported \n \t Supported platforms: Linux",
                    platform
                )
            }
        }
    }
}
