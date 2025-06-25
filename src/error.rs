use std::fmt;

#[derive(Debug)]
pub enum JigError {
    Config(String),
    Io(std::io::Error),
    NotImplemented(String),
    Other(String),
}

impl fmt::Display for JigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JigError::Config(msg) => write!(f, "Configuration error: {}", msg),
            JigError::Io(err) => write!(f, "IO error: {}", err),
            JigError::NotImplemented(cmd) => write!(f, "Command not implemented: {}", cmd),
            JigError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for JigError {}

impl From<std::io::Error> for JigError {
    fn from(err: std::io::Error) -> Self {
        JigError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, JigError>;
