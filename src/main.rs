use std::env;

mod arg_parser;

fn main() {
    println!("Hello, world!");

    let arg = arg_parser::parse_args();
    match arg {
        Ok(arg) => println!("{}", arg),
        Err(e) => println!("FAILED: {}", e),
    }
}
