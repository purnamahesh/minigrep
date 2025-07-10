use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {

    let args:Vec<String> = env::args().collect();

    let config = Config::build(&args)
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("\nIGNORE_CASE={}",config.ignore_case);
    println!("\nSearching for {}", config.query);
    println!("\nIn file {}\n", config.file_name);

    if let Err(e) = run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
