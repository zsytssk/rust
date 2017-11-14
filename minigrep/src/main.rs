extern crate minigrep;

use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        panic!("problem parsing arguments:{}", err);
    });

    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}
