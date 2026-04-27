use std::{env, error::Error};

use ccwc;

fn main() {
    let result = run();
    match result {
        Ok(s) => println!("Result: {}", s),
        Err(e) => println!("Failed: {}", e),
    }
}

fn run() -> Result<String, Box<dyn Error>> {
    let arg = ccwc::arg_parser::parse_args()?;
    let reader = arg.input_source.read()?;
    let result = arg.operation.run(reader)?;
    Ok(format!("Op: {}, Result: {}", arg.operation, result))
}
