use std::env;
use std::fs;

fn main() {
    // reading the argument values
    let args: Vec<String> = env::args().collect();

    // saving the argument values
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // reading the file
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// grouping configuration values
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // constructor for Config
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
