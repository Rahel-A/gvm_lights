use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum GvmNodeError {
    NodesNotFound,
    StateRetrievalFailed,
    IO(std::io::Error),
}

impl std::error::Error for GvmNodeError {
    fn description(&self) -> &str {
        match self {
            GvmNodeError::NodesNotFound => "no known nodes",
            GvmNodeError::StateRetrievalFailed => "failed to retrieve states from gvm nodes",
            GvmNodeError::IO(_) => "IO error",
        }
    }
}

impl From<std::io::Error> for GvmNodeError {
    fn from(err: std::io::Error) -> GvmNodeError {
        GvmNodeError::IO(err)
    }
}

impl fmt::Display for GvmNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::error::Error::description(&self).fmt(f)
    }
}
