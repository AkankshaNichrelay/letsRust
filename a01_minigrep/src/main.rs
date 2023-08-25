use std::env;
use std::process;
use a01_minigrep as minigrep;
use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
