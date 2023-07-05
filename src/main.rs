use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1)
    });

    println!("Searching for {}",config.query);
    println!("In file {}",config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1)
    }
}



