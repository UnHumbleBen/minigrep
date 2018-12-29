use std::error::Error;
use std::fs;

// grouping configuration values
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // constructor for Config
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // error message
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// extracting error from run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // reading the file
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
