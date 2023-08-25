use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("Searching for {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");

    println!("file contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone(); // because we dont want ownership of these strings
    let filename = args[2].clone();
    // if we wanted to store references to strings, we would need to use lifetimes.
    Config { query, filename }
}