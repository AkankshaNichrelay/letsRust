# Rust File System

Cargo init creates a new package. Each package has crates. Each crate has modules that allow you to scope and organize a chunk of code and control privacy rules.

We can define our crates in Cargo.toml. By default we have a binary crate. If our src has main.rs defined, then a binary crate with the same name as your packge will be created automatically. The main.rs will be the crate root. A crate root is the source file that the rust complier starts at when building your crate. It also makes up the root module of your crate. 

We have same convention for library crates. When we have lib.rs defined in the root of your src directory, then rust will automatically create a library crate with the same name as your package and lib.rs will be the crate root.

## Rules for crates
1. A package must have alteast one crate.
2. A package could have either zero or one library crate.
3. A package could have any number of binary crates.

If we want more library crates, we would create a folder under src -> bin. Each file defined in bin folder will represent a binary crate.

