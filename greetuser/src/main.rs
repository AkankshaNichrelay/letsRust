// use reduces the namespace usage.
use std::io;    // import the io namespace from the standard lib

// Strings in Rust is funky
fn greet_user() -> String {
    println!("Hello, what is your name?");
    let mut buffer = String::new();     // Create a MUTABLE string
    let stdin = io::stdin();            // Acquire standard input from OS
    
    // Rust doesnt have exceptions; when a rust function fails, it can return a result.
    // The result can be okay and wraps contents Or it can be an error.
    //  Unwrap crashes the program in case of an error
    stdin.read_line(&mut buffer).unwrap();  
    // by default Rust functions return the result of the last expression.
    buffer
}

fn main() {
    // let user_name = greet_user(); also works
    let user_name: String = greet_user();
    println!("Hello, {user_name}");
}