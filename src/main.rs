use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {:?}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);
    if let Err(e) = run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


