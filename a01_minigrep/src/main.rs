use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("Searching for {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    println!("file contents: {}", contents);
}

fn parse_config(args: &Vec<String>) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}