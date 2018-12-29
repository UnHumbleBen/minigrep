use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // reading the argument values
    let args: Vec<String> = env::args().collect();

    // saving the argument values
    let config = Config::new(&args).unwrap_or_else(|err| {
        // handling error
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // handling error from run function
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
