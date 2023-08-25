use std::env;
use std::fs;
use std::process;
use std::error::Error;

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

    // if the call to run results in an error variant then execute the code inside this block.
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// here we are saying return any type of error
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator is used for early exit an function with return type that's compatible
    //  with the value of the ? is used on
    let contents = fs::read_to_string(config.filename)?;
    println!("file contents: {}", contents);
    Ok(())
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

