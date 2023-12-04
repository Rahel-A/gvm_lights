use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum GvmServerError {
    EndOfStream,
    ParsingError,
    NodesNotFound,
    IO(std::io::Error),
}

impl From<std::io::Error> for GvmServerError {
    fn from(err: std::io::Error) -> GvmServerError {
        GvmServerError::IO(err)
    }
}

impl fmt::Display for GvmServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GvmServerError::EndOfStream => "protocol error".fmt(f),
            GvmServerError::ParsingError => "de-/serializing failed".fmt(f),
            GvmServerError::NodesNotFound => "no gvm light devices found".fmt(f),
            GvmServerError::IO(_) => "IO error".fmt(f),
        }
    }
}

impl std::error::Error for GvmServerError {}

pub fn bincode_to_io_error(_err: Box<bincode::ErrorKind>) -> std::io::Error {
    Error::new(ErrorKind::Other, "Failed to serialize command")
}
