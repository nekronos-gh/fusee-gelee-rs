use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub enum RcmError {
    DeviceNotFound,
    PayloadNotFound(PathBuf),
    PayloadTooLarge(u16),
    UnsupportedPlatform(String),
}

impl fmt::Display for RcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RcmError::DeviceNotFound => write!(f, "No device in RCM found"),
            RcmError::PayloadNotFound(p) => write!(f, "Payload {} not found", p.display()),
            RcmError::PayloadTooLarge(s) => write!(f, "Payload is too large: {} Bytes", s),
            RcmError::UnsupportedPlatform(s) => {
                write!(
                    f,
                    "Platform \"{}\" is not supported \n \t Supported platforms: Linux",
                    s
                )
            }
        }
    }
}
