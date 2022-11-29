////////////////////////////////////////////////////////////////////////////////
// Paths for Referring to an Item in the Module Tree
////////////////////////////////////////////////////////////////////////////////
mod paths_for_referring_to_an_item_in_the_module_tree {
    //! A path can take two forms:
    //!   * An absolute path is the full path starting from a `crate` root; for code from an
    //!   external `crate`, the absolute path begins with the `crate` name, and for code from the
    //!   current `crate`, it starts with the literal `crate`.
    //!   * A relative path starts from the current module and uses `self`, `super`, or an
    //!   identifier in the current module.
}

////////////////////////////////////////////////////////////////////////////////
// Best Practices for Packages with a Binary and a Library
////////////////////////////////////////////////////////////////////////////////
mod best_practices_for_packages_with_a_binary_and_a_library {
    //! The module tree should be defined in src/lib.rs. Then, any public items can be used in the
    //! binary crate by starting paths with the name of the package. The binary crate becomes a user
    //! of the library crate just like a completely external crate would use the library crate.
}

////////////////////////////////////////////////////////////////////////////////
// Starting Relative Paths with super
////////////////////////////////////////////////////////////////////////////////
mod starting_relative_paths_with_super {
    //! We can construct relative paths that begin in the parent module, rather than the current
    //! module or the `crate` root, by using `super` at the start of the path. Using `super` allows
    //! us to reference an item that we know is in the parent module.

    #[allow(dead_code)]
    fn deliver_order() {}

    mod back_of_house {
        #[allow(dead_code)]
        fn cook_order() {}

        #[allow(dead_code)]
        fn fix_incorrect_order() {
            cook_order();
            super::deliver_order();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Bringing Paths into Scope with the use Keyword
////////////////////////////////////////////////////////////////////////////////
mod bringing_paths_into_scope_with_the_use_keyword {
    //! `use` only creates the shortcut for the particular scope in which the `use` occurs.

    mod front_of_house {
        pub mod hosting {
            #[allow(dead_code)]
            pub fn add_to_wait_list() {}
        }
    }

    use front_of_house::hosting;

    #[allow(dead_code)]
    pub fn eat_at_restaurant() {
        hosting::add_to_wait_list();
    }

    mod customer {
        use super::front_of_house::hosting as customer_hosting;

        #[allow(dead_code)]
        pub fn eat_at_restaurant() {
            // hosting is not valid here, you can refer to hosting like:
            // hosting::add_to_wait_list();

            customer_hosting::add_to_wait_list();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Providing New Names with the as Keyword
////////////////////////////////////////////////////////////////////////////////
mod providing_new_names_with_the_as_keyword {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    #[allow(dead_code)]
    fn function1() -> Result {
        Ok(())
    }

    #[allow(dead_code)]
    fn function2() -> IoResult<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Re-exporting Names with pub use
////////////////////////////////////////////////////////////////////////////////
mod re_exporting_names_with_pub_use {
    //! When we bring a name into scope with the `use` keyword, the name available in the new scope
    //! is private. To enable the code that calls our code to refer to that name as if it had been
    //! defined in that code’s scope, we can combine `pub` and `use`. This technique is called
    //! re-exporting because we’re bringing an item into scope but also making that item available
    //! for others to bring into their scope.
    //!
    //! # Examples
    //!
    //! ```
    //! mod front_of_house {
    //!     pub mod hosting {
    //!         pub fn add_to_wait_list() {}
    //!     }
    //! }
    //!
    //! pub use crate::front_of_house::hosting;
    //!
    //! pub fn eat_at_restaurant() {
    //!     hosting::add_to_wait_list();
    //! }
    //! ```
    //!
    //! Before this change, external code would have to call the `add_to_wait_list` function by
    //! using the path `restaurant::front_of_house::hosting::add_to_wait_list()`. Now that this
    //! `pub use` has re-exported the hosting module from the root module, external code can now
    //! use the path `restaurant::hosting::add_to_wait_list()` instead.
}

////////////////////////////////////////////////////////////////////////////////
// Using External Packages
////////////////////////////////////////////////////////////////////////////////
mod using_external_packages {
    //! Use external package such as `rand`, we add this line to `Cargo.toml`: rand = "0.8.3".
    //! Adding `rand` as a dependency in `Cargo.toml` tells Cargo to download the rand package and
    //! any dependencies from crates.io and make rand available to our project.
}

////////////////////////////////////////////////////////////////////////////////
// Using Nested Paths to Clean Up Large use Lists
////////////////////////////////////////////////////////////////////////////////
mod using_nested_paths_to_clean_up_large_use_lists {
    //! If we’re using multiple items defined in the same `crate` or same module, listing each item
    //! on its own line can take up a lot of vertical space in our files. Instead, we can use nested
    //! paths to bring the same items into scope in one line. We do this by specifying the common
    //! part of the path, followed by two colons, and then curly brackets around a list of the parts
    //! of the paths that differ.
    //!
    //! # Examples
    //!
    //! ```
    //! use std::cmp::Ordering;
    //! use std::io;
    //! ```
    //!
    //! ```
    //! use std::{cmp::Ordering, io};
    //! ```
    //!
    //! ```
    //! use std::io;
    //! use std::io::Write;
    //! ```
    //!
    //! ```
    //! use std::io::{self, Write};
    //! ```
}

////////////////////////////////////////////////////////////////////////////////
// Glob Operator
////////////////////////////////////////////////////////////////////////////////
mod glob_operator {
    //! If we want to bring all public items defined in a path into scope, we can specify that path
    //! followed by the `*` glob operator.
    //!
    //! # Examples
    //!
    //! ```
    //! use std::collections::*;
    //! ```
}
