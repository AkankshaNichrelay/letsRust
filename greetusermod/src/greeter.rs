use std::io; 

// everything is private by default in rust until specified otherwise
pub fn greet_user() -> String {
    println!("Hello, what is your name?");
    let mut buffer = String::new();     
    let stdin = io::stdin();            
    
    //  Unwrap crashes the program in case of an error
    stdin.read_line(&mut buffer).unwrap(); 
    buffer
}