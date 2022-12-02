//! # Error
//!
//! Rust groups errors into two major categories: recoverable and unrecoverable errors.
//!
//! Rust does not have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors
//! and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

pub mod unwind_stack {
    //! By default, when a panic occurs, the program starts unwinding, which means Rust walks back
    //! up the stack and cleans up the data from each function it encounters. However, this walking
    //! back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of
    //! immediately aborting, which ends the program without cleaning up.
    //!
    //! Memory that the program was using will then need to be cleaned up by the operating system.
    //! If in your project you need to make the resulting binary as small as possible, you can
    //! switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the
    //! appropriate [profile] sections in your Cargo.toml file.
    //!
    //! For example, if you want to abort on panic in release mode, add this:
    //! ```Cargo.toml
    //! [profile.release]
    //! panic = 'abort'
    //! ```
    //!
    //! For example, if you want to abort on panic in debug mode, add this:
    //! ```Cargo.toml
    //! [profile.dev]
    //! panic = 'abort'
    //! ```
    //!
    //! Then, write a simple case to show the difference between `panic = 'abort'` and without it.
    //!
    //! ```panic
    //! fn main() {
    //!     let a: i8 = 1;
    //!     let b: i8 = 0;
    //!     let c: i8 = a / b;
    //! }
    //! ```
    //!
    //! ```shell
    //! RUST_BACKTRACE=full cargo run
    //! ```
}

pub mod panic {
    //! There are two ways to cause a panic in practice: by taking an action that causes our code
    //! to panic(such as accessing an array past the end) or by explicitly calling the panic! macro.

    pub fn takes_action_to_cause_panic() {
        // let _ = 1 / 0;
    }

    pub fn explicitly_call_panic_marco() {
        // panic!("crash and burn");
    }
}

pub mod result {
    use std::fs::File;
    use std::io::{Result, Error};

    /// The return type of `File::open` is a `Result<T, E>`. The generic parameter `T` has been
    /// filled in by the implementation of `File::open` with the type of the success value,
    /// `std::fs::File`, which is a file handle. The type of `E` used in the error value is
    /// `std::io::Error`.
    pub fn match_to_handle_result() {
        let mut file_result: Result<File> = File::open("rust.rs");
        let _file = match file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    pub fn match_on_different_errors() {
        let file_result: Result<File> = File::open("rust.rs");
    }
}

mod testing {
    #[test]
    fn run_result_open_file() {
        crate::result::match_to_handle_result();
    }
}
