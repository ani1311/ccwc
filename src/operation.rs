use std::{
    fmt,
    io::{self, BufRead},
};

mod count_bytes;
mod count_lines;
mod count_words;

pub enum Operation {
    CountBytes,
    CountLines,
    CountWords,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountBytes => write!(f, "Count Bytes"),
            Self::CountLines => write!(f, "Count Lines"),
            Self::CountWords => write!(f, "Count Words"),
        }
    }
}

impl Operation {
    pub fn run<R: BufRead>(&self, reader: R) -> io::Result<usize> {
        match self {
            Self::CountBytes => count_bytes::run(reader),
            Self::CountLines => count_lines::run(reader),
            Self::CountWords => count_words::run(reader),
        }
    }
}
