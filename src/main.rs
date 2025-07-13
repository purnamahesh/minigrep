use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {

    let args = env::args();

    let config = Config::build(args)
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "\tIGNORE_CASE={}\tquery={}\tfile={}\n",
        config.ignore_case, 
        config.query, 
        config.file_name
    );

    if let Err(e) = run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
