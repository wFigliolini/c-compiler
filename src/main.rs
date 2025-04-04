use std::{env, process};

use c_compiler::Config;

fn main() {
    let _config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
