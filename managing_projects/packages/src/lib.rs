////////////////////////////////////////////////////////////////////////////////
// Manage Packages
////////////////////////////////////////////////////////////////////////////////
mod manage_packages {
    //! A package can contain as many binary crates as you like, but at most only one library crate.
    //! A package must contain at least one crate, whether that’s a library or binary crate.
    //! ```text
    //! package1
    //! |── src
    //!     |── main.rs
    //! |── Cargo.lock
    //! |── Cargo.toml
    //!
    //! package2
    //! |── src
    //!     |── lib.rs
    //! |── Cargo.lock
    //! |── Cargo.toml
    //!
    //! package3
    //! |── src
    //!     |── lib.rs
    //!     |── main.rs
    //! |── Cargo.lock
    //! |── Cargo.toml
    //!
    //! package4
    //! |── src
    //!     |── bin
    //!         |── main1.rs
    //!         |── main2.rs
    //!     |── lib.rs
    //!     |── main.rs
    //! |── Cargo.lock
    //! |── Cargo.toml
    //!
    //! package5
    //! |── src
    //!     |── bin
    //!         |── main1.rs
    //!         |── main2.rs
    //!     |── garden
    //!         |── vegetable.rs
    //!     |── garden.rs
    //!     |── lib.rs
    //!     |── main.rs
    //! |── Cargo.lock
    //! |── Cargo.toml
    //! ```
    //!
    //! Let’s walk through what happens when we create a package. First, we enter the command to
    //! crate a new package:
    //! ```shell
    //! $ cargo new my-project
    //!      Created binary (application) `my-project` package
    //! $ ls my-project
    //! Cargo.toml
    //! src
    //! $ ls my-project/src
    //! main.rs
    //! ```
    //!
    //! Cargo follows a convention that src/main.rs is the crate root of a binary crate with the
    //! same name as the package. Likewise, Cargo knows that if the package directory contains
    //! src/lib.rs, the package contains a library crate with the same name as the package, and
    //! src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the
    //! library or binary.
    //!
    //! Here, we have a package that only contains src/main.rs, meaning it only contains a binary
    //! crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates:
    //! a binary and a library, both with the same name as the package. A package can have multiple
    //! binary crates by placing files in the src/bin directory: each file will be a separate binary
    //! crate.
}
