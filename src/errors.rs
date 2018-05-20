use std::convert;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
    IoError(io::Error),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            InterpretError::CompileError(details) => write!(f,"Compile error: {}", details),
            InterpretError::RuntimeError(details) => write!(f,"Runtime error: {}", details),
            InterpretError::IoError(io_error) => write!(f,"IO error: {}", io_error),
        }
    }
}

impl Error for InterpretError {
    fn description(&self) -> &str {
        match &self {
            InterpretError::CompileError(details) => &details,
            InterpretError::RuntimeError(details) => &details,
            InterpretError::IoError(io_error) => &io_error.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match &self {
            InterpretError::IoError(io_error) => Some(io_error),
            _ => None,
        }
    }
}

impl convert::From<io::Error> for InterpretError {
    fn from(err: io::Error) -> InterpretError {
        InterpretError::IoError(err)
    }
}

pub type InterpretResult<T> = Result<T, InterpretError>;
