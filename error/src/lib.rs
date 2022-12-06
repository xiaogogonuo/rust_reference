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
    use std::fs::{self, File};
    use std::io::{self, Error, ErrorKind, Read};

    /// The return type of `File::open` is a `Result<T, E>`. The generic parameter `T` has been
    /// filled in by the implementation of `File::open` with the type of the success value,
    /// `std::fs::File`, which is a file handle. The type of `E` used in the error value is
    /// `std::io::Error`.
    pub fn match_to_handle_result() {
        let file_result: io::Result<File> = File::open("not_exist");
        let _file = match file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    pub fn match_on_different_errors() {
        let file_result: io::Result<File> = File::open("rust.rs");
        let _file = match file_result {
            Ok(f) => f,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("rust.rs") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    /// The return type of `File::open()` is `Result<T, E>`, which has a method `unwrap_or_else`.
    /// If the Result value is the `Ok` variant, it returns the value inside the `Ok`, If the Result
    /// is the `Err` variant, deal the error with closure.
    pub fn match_with_result_closure() {
        let _file = File::open("rust.rs").unwrap_or_else(|error: Error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("rust.rs").unwrap_or_else(|error: Error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    /// If the Result value is the `Ok` variant, `unwrap` will return the value inside the `Ok`.
    /// If the Result is the `Err` variant, `unwrap` will call the `panic!` macro.
    ///
    /// Similarly, the `expect` method lets us also choose the `panic!` error message.
    pub fn shortcut_for_panic_on_error() {
        let _file: File = File::open("not_exist").unwrap();

        let _file: File =
            File::open("not_exist").expect("not_exist should be included in this project");
    }

    /// When a function’s implementation calls something that might fail, instead of handling the
    /// error within the function itself, you can return the error to the calling code so that it
    /// can decide what to do. This is known as propagating the error and gives more control to the
    /// calling code.
    pub fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result: io::Result<File> = File::open("not_exist");
        let mut username_file: File = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username: String = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    /// A Shortcut for Propagating Errors: the `?` Operator
    ///
    /// If the value of the Result is an `Ok`, the value inside the `Ok` will get returned from this
    /// expression, and the program will continue. If the value is an `Err`, the `Err` will be
    /// returned from the whole function as if we had used the return keyword.
    pub fn shortcut_for_propagating_with_question_mark_operation() -> Result<String, io::Error> {
        let mut username_file: File = File::open("not_exist")?;
        let mut username: String = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    /// The `?` operator in a function that returns `Result`, `Option`, or another type that
    /// implements `FromResidual`.
    pub fn chaining_method_calls_after_question_mark_operator() -> Result<String, io::Error> {
        let mut username: String = String::new();
        File::open("not_exist")?.read_to_string(&mut username)?;
        Ok(username)
    }

    pub fn convenient_function_provided_by_std() -> Result<String, io::Error> {
        fs::read_to_string("not_exist")
    }
}

mod testing {
    #[test]
    #[should_panic]
    fn run_result_open_file() {
        crate::result::match_to_handle_result();
    }

    #[test]
    fn run_result_match_on_different_errors() {
        crate::result::match_on_different_errors();
    }

    #[test]
    fn run_result_match_with_result_closure() {
        crate::result::match_with_result_closure();
    }

    #[test]
    #[should_panic]
    fn run_result_shortcut_for_panic_on_error() {
        crate::result::shortcut_for_panic_on_error()
    }
}
