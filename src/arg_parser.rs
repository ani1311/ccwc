use crate::{input_source, operation::Operation};
use std::{
    env,
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug)]
pub enum ArgError {
    NoArg,
    UnknownArg(String),
    BadArgs(String),
}

pub struct ParsedArgs {
    pub operation: Operation,
    pub input_source: input_source::InputSource,
}

impl Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArg => write!(f, "No args"),
            Self::UnknownArg(s) => writeln!(f, "Unknown arg {}", s),
            Self::BadArgs(s) => writeln!(f, "Bad args: {}", s),
        }
    }
}

impl Error for ArgError {}

pub fn parse_args() -> Result<ParsedArgs, ArgError> {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    match args.as_slice() {
        [_program] => Err(ArgError::NoArg),
        [_program, "-c", path] => Ok(ParsedArgs {
            operation: Operation::CountBytes,
            input_source: input_source::InputSource::File(std::path::PathBuf::from(path)),
        }),
        [_program, "-l", path] => Ok(ParsedArgs {
            operation: Operation::CountLines,
            input_source: input_source::InputSource::File(std::path::PathBuf::from(path)),
        }),
        [_program, "-w", path] => Ok(ParsedArgs {
            operation: Operation::CountWords,
            input_source: input_source::InputSource::File(std::path::PathBuf::from(path)),
        }),
        a => Err(ArgError::BadArgs(a.join(" "))),
    }
}
