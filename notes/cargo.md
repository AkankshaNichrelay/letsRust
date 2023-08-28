# Cargo

## Tools
* Rustup - managaing installations over time
* Cargo - does everything
* Rustc - cargo calls it over time; for imprting outside packages if not fully Rust
* Clippy - runs in background; makes suggestions
* Cargo's sub tools

### Stay up-to-date with RustUp

* `rustup self update`
* `rustup update`

### Cargo
Cargo is how you interact with Rust in a useful fashion
``` cargo init hello ```
It tells cargo that you would like to create a new project. It creates a new folder Hello. The Hellor folder contains a src main file with basic hello world print statement. Cargo also creates a .git folder by default.

Cargo also creates a toml file. It describes how your program should be build. Cargo.toml has to exist for every Rust project. Cargo uses it to determine what your project is and how it should be compiled.

