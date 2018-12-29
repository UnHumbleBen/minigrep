use std::env;
use std::fs;

fn main() {
    // reading the argument values
    let args: Vec<String> = env::args().collect();

    // saving the argument values
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // reading the file
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
