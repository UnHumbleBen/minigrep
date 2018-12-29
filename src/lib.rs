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

    // using the search function
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // storing matching lines
    let mut results = Vec::new();

    //iterating through the lines
    for line in contents.lines() {
        // searching for query
        if line.contains(query) {
            results.push(line); 
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
