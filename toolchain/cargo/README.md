# check whether `cargo` is installed
```shell
cargo --version
```

# create a new binary crate
```shell
cargo new xxx
```

# create a new library crate
```shell
cargo new xxx --lib
```

# build for debug
```shell
cargo build
```
* create an executable file in target/debug/project

# build for release
```shell
cargo build --release
```
* create an executable file in target/release/project
* compile with optimizations, makes code run faster, but takes more time to compile

# compile the code and then run the resulting executable
```shell
cargo run
```

# check your code to make sure it compiles but does not produce an executable
```shell
cargo check
```

# delete target directory
```shell
cargo clean
```

# build documentation provided by all of your dependencies locally and open it in your browser
```shell
cargo doc --open
```

# format all binary and library targets of your crate
```shell
cargo fmt
```

# 
```shell
cargo test -- --show-output
```