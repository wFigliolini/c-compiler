use std::{env, process};

use c_compiler::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = c_compiler::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
