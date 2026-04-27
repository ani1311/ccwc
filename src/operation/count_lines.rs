use std::io::{BufRead, Result};

pub fn run<R: BufRead>(reader: R) -> Result<usize> {
    Ok(reader.lines().count())
}
