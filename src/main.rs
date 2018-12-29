use std::env;
use std::fs;
use std::process;
use std::error::Error;

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// extracting error from run function
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // reading the file
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
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
