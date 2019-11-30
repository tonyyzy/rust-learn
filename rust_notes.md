# Some notes while learning rust

- define function, entry point is `main()`
    ```rust
    fn main() {

    }
    ```

- stdout with `println!()`

- Compile with `rustrc *.rs`

- Some `cargo` usages
    ```bash
    cargo new *name* # initiate new project
    cargo build # build project
    cargo run # run binary
    cargo check # check compilation without produce binary
    ```
- Input/output library from standard library `use std::io;`

- Use `let` to create variables; use `mut` to specify mutatble variables

- `String::new()` returns a new instance of a `String`

- `//` to start a comment
