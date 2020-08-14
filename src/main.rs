mod lib;
use std::env;
use std::process;

use lib::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument : {}", err);
        process::exit(1);
    });

    println!("Searcing for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Error :{}", e);
        process::exit(1);
    }
}

