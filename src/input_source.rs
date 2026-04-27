use std::{
    fs,
    io::{BufRead, BufReader, Read, Result},
    path::PathBuf,
};

pub enum InputSource {
    File(PathBuf),
}

impl InputSource {
    pub fn read(&self) -> Result<impl BufRead> {
        match self {
            InputSource::File(p) => read(p),
        }
    }
}

fn read(p: &PathBuf) -> Result<impl BufRead> {
    let f = fs::File::open(p)?;
    Ok(BufReader::new(f))
}
