use std::env;
use std::fs;
use std::process;

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // error message
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
