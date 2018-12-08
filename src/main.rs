extern crate patterns;

use std::env;
use std::process;
use patterns::Config;

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("in file {}", config.filename);
    if let Err(e) = patterns::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}