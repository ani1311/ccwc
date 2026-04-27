use std::io::{BufRead, Result};

pub fn run<R: BufRead>(reader: R) -> Result<usize> {
    let mut count = 0;
    for line in reader.lines() {
        count += line?.split_whitespace().count();
    }

    Ok(count)
}
