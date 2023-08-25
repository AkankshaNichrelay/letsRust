use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    );

    println!("Searching for {}", config.query);
    println!("Searching for {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename);
        .expect("Something went wrong reading thefile");

    println!("file contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // panic is better for programming error and not user error
            // panic!("not enough arguments");
            return Err("not enough arguments")
        }

        let query = args[1].clone(); // because we dont want ownership of these strings
        let filename = args[2].clone();
        // if we wanted to store references to strings, we would need to use lifetimes.
        Ok(Config { query, filename })
    }
}

