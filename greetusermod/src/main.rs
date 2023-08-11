mod greeter;       //include "greeter.rs" in the build
use greeter::greet_user;  // import greet_user from greeter module


fn main() {
    let user_name: String = greet_user();
    println!("Hello, {user_name}");
}