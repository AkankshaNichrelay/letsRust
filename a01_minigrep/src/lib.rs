// this is the root of our library crate
use std::fs;
use std::error::Error;

// here we are saying return any type of error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator is used for early exit an function with return type that's compatible
    //  with the value of the ? is used on
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);    
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;       // importing everything from parent module

    #[test]         // test attribute
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}