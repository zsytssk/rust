extern crate minigrep;

use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
