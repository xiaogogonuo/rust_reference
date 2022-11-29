//! # References and Borrowing
//!
//! Reference is an address like a pointer, we can follow to access the data stored at that address.
//! Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for
//! the life of that reference.
//!
//! Just as variables are immutable by default, so are references. We’re not allowed to modify
//! something we have a reference to.
//!
//! Note that a reference’s scope starts from where it is introduced and continues through the last
//! time that reference is used. For instance, this code will compile because the last usage of the
//! immutable references, the println!, occurs before the mutable reference is introduced:
//!
//! # Examples
//!
//! ```
//! let mut s = String::from("hello");
//!
//! let r1 = &s; // no problem
//! let r2 = &s; // no problem
//! println!("{} and {}", r1, r2);
//! // variables r1 and r2 will not be used after this point
//!
//! let r3 = &mut s; // no problem
//! println!("{}", r3);
//! ```
//!
//! The scopes of the immutable references r1 and r2 end after the println! where they are last
//! used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this
//! code is allowed. The ability of the compiler to tell that a reference is no longer being used
//! at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short).

#[allow(dead_code)]
fn string_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to,
  // the value it points to will not be dropped when the reference stops being used.

mod mutable_reference {
    //! Mutable references have one big restriction: if you have a mutable reference to a value,
    //! you can have no other references to that value.
    //!
    //! The benefit of having this restriction is that Rust can prevent data races at compile time.

    /// mutable reference and immutable reference can declared both, and compile won't panic if they
    /// are not used.
    #[allow(dead_code)]
    pub fn weak_restriction() {
        let mut s: String = String::from("rust");
        let _r0: &String = &s;
        let _r1: &mut String = &mut s;
        let _r2: &mut String = &mut s;
        let _r3: &String = &s;
    }

    /// ‼️ essence ‼️
    /// Mutable reference's lifetime can't have a cross with other references' lifetime, no matter
    /// other references are mutable reference or immutable reference.
    #[allow(dead_code)]
    pub fn strong_restriction() {
        let mut s: String = String::from("rust");

        // succeed
        {
            let r1: &String = &s;
            let r2: &String = &s;
            println!("{}", r2);
            println!("{}", r1);
        }

        // succeed
        {
            let r1: &String = &s;
            println!("{}", r1);
            let r2: &mut String = &mut s;
            println!("{}", r2);
        }

        // succeed
        {
            let r1: &mut String = &mut s;
            println!("{}", r1);
            let r2: &String = &s;
            println!("{}", r2);
        }

        // succeed
        {
            let r1: &mut String = &mut s;
            println!("{}", r1);
            let r2: &mut String = &mut s;
            println!("{}", r2);
        }

        // panic
        // {
        //     let r1: &mut String = &mut s;
        //     let r2: &mut String = &mut s;
        //     println!("{}", r2);
        //     println!("{}", r1);
        // }

        // panic
        // {
        //     let r1: &String = &s;
        //     let r2: &mut String = &mut s;
        //     println!("{}", r2);
        //     println!("{}", r1);
        // }

        // panic
        // {
        //     let r1: &mut String = &mut s;
        //     let r2: &String = &s;
        //     println!("{}", r2);
        //     println!("{}", r1);
        // }
    }
}

mod dangling_reference {
    //! A pointer that references a location in memory that may have been given to someone else.
    //! In Rust, the compiler guarantees that references will never be dangling references: if you
    //! have a reference to some data, the compiler will ensure that the data will not go out of
    //! scope before the reference to the data does.
    //!
    //! ```text
    //! pub fn dangling() -> &String { // dangle returns a reference to a String
    //!     let s = "rust".to_string();
    //!     &s // we return a reference to the String, s
    //! } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //! ```
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_mutable_reference_restriction() {
        crate::mutable_reference::weak_restriction();
        crate::mutable_reference::strong_restriction();
    }
}
