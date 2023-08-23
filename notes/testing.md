## Testing in Rust

* In Rust, you can write document tests as well. 

* A test fails when something inside the test function panics. Each test is ran in a new thread and if the main thread sees that a test thread has died, then the main thread fails the test.

* Both the parameters passed into `assert_eq!` or `assert_ne!` have to implement the partial equal and debug traits. All the primitive values and most of the standard library have these two traits implemented. But if we create our own structures or enums, we will have to implement these ourselves.

* `cargo test` compiles your code in test mode and runs the resulting test binary. By default all tests are run in parallel in a separate thread. Also, all generated output is captured and not printed to the screen. So any print statements wont print.

* You can specify command line options for both cargo test command and the test binary, separated by `--`. `cargo test -- --help`

* specify threads: `cargo test -- --test-threads=1`

* show print statements for passing tests: `cargo test -- --show-output`

* You cannot directly test binary crates with integration tests. So it is common to see a binary crate that is a very thin wrapper around a library crate so that you can write integration tests for the library crate. 
