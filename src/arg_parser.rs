use std::env;
use std::fmt::Error;

pub fn parse_args() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        return;
    }

    todo!()
}
