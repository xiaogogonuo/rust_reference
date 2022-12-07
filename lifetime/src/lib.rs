//! # Lifetimes
//!
//! Lifetimes are another kind of generic. Rather than ensuring that a type has the behavior we
//! want, lifetimes ensure that references are valid as long as we need them to be.
//!
//! Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
//!
//! Most of the time, lifetimes are implicit and inferred. We only must annotate types when multiple
//! types are possible.
//!
//! Rust requires us to annotate the relationships using generic lifetime parameters to ensure the
//! actual references used at runtime will definitely be valid.
//!
//! The main aim of lifetimes is to prevent dangling references.

pub mod borrow_checker {
    //! The Rust compiler has a borrow checker that compares scopes to determine whether all borrows
    //! are valid.
    //!
    //! Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you see,
    //! the inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust
    //! compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it
    //! refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than
    //! 'a: the subject of the reference does not live as long as the reference.
    //!
    //! ```panic
    //! fn main() {
    //!     let r;                // ---------+-- 'a
    //!                           //          |
    //!     {                     //          |
    //!         let x = 5;        // -+-- 'b  |
    //!         r = &x;           //  |       |
    //!     }                     // -+       |
    //!                           //          |
    //!     println!("r: {}", r); // ---------+
    //! }
    //! ```
    //!
    //! Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can
    //! reference x because Rust knows that the reference in r will always be valid when x is valid.
    //! ```
    //! fn main() {
    //!     let x = 5;            // ----------+-- 'b
    //!                           //           |
    //!     let r = &x;           // --+-- 'a  |
    //!                           //   |       |
    //!     println!("r: {}", r); // --+       |
    //!                           //           |
    //! }                         // ----------+
    //! ```
}

pub mod generic_lifetime_in_function {
    //! If we try to implement the longest function as shown, it won’t compile.
    //!
    //! ```panic
    //! fn longest(x: &str, y: &str) -> &str {
    //!     if x.len() > y.len() {
    //!         x
    //!     } else {
    //!         y
    //!     }
    //! }
    //! ```
    //!
    //! Instead, we get the following error that talks about lifetimes:
    //! ```shell
    //! $ cargo run
    //!    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
    //! error[E0106]: missing lifetime specifier
    //!  --> src/main.rs:9:33
    //!   |
    //! 9 | fn longest(x: &str, y: &str) -> &str {
    //!   |               ----     ----     ^ expected named lifetime parameter
    //!   |
    //!   = help: this function's return type contains a borrowed value, but the signature does not
    //! say whether it is borrowed from `x` or `y`
    //! help: consider introducing a named lifetime parameter
    //!   |
    //! 9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //!   |           ++++     ++          ++          ++
    //!
    //! For more information about this error, try `rustc --explain E0106`.
    //! error: could not compile `chapter10` due to previous error
    //! ```
    //!
    //! The help text reveals that the return type needs a generic lifetime parameter on it because
    //! Rust can’t tell whether the reference being returned refers to x or y.
    //!
    //! When we’re defining this function, we don’t know the concrete values that will be passed
    //! into this function, so we don’t know whether the if case or the else case will execute.
    //! We also don’t know the concrete lifetimes of the references that will be passed in, The
    //! borrow checker can’t determine this either, because it does not know how the lifetimes of
    //! x and y relate to the lifetime of the return value.
}

pub mod lifetime_annotation_syntax {
    //! Lifetime annotations don’t change how long any of the references live. Rather, they describe
    //! the relationships of the lifetimes of multiple references to each other without affecting
    //! the lifetimes.
    //!
    //! Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must
    //! start with an apostrophe `'` and are usually all lowercase and short, like generic types.
    //! Most people use the name 'a for the first lifetime annotation. We place lifetime parameter
    //! annotations after the & of a reference, using a space to separate the annotation from the
    //! reference’s type.
    //!
    //! Here are some examples:
    //!
    //! ```text
    //! &i32        // a reference
    //! &'a i32     // a reference with an explicit lifetime
    //! &'a mut i32 // a mutable reference with an explicit lifetime
    //! ```
}

pub mod lifetime_annotation_in_function_signature {
    //! To use lifetime annotations in function signatures, we need to declare the generic lifetime
    //! parameters inside angle brackets between the function name and the parameter list, just as
    //! we did with generic type parameters.

    /// The `longest` function definition specifying that all the references in the signature must
    /// have the same lifetime `'a`.
    ///
    /// The function signature now tells Rust that for some lifetime `'a`, the function takes two
    /// parameters, both of which are string slices that live at least as long as lifetime `'a`.
    /// The function signature also tells Rust that the string slice returned from the function will
    /// live at least as long as lifetime `'a`.
    ///
    /// In practice, it means that the lifetime of the reference returned by the longest function is
    /// the same as the smaller of the lifetime of the values referred to by the function arguments.
    ///
    /// Remember, when we specify the lifetime parameters in this function signature, we’re not
    /// changing the lifetimes of any values passed in or returned. Rather, we’re specifying that
    /// the borrow checker should reject any values that don’t adhere to these constraints.
    ///
    /// When we pass concrete references to longest, the concrete lifetime that is substituted for
    /// `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words,
    /// the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of
    /// the lifetimes of `x` and `y`. Because we’ve annotated the returned reference with the same
    /// lifetime parameter `'a`, the returned reference will also be valid for the length of the
    /// smaller of the lifetimes of `x` and `y`.
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

pub mod thinking_in_terms_of_lifetime {

    /// If we changed the implementation of the `longest` function to always return the first
    /// parameter rather than the longest string slice, we do not need to specify a lifetime on the
    /// `y` parameter. The following code will compile.
    pub fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }
}

pub mod lifetime_annotation_in_struct {
    //! So far, the structs we’ve defined all hold owned types. We can define structs to hold
    //! references, but in that case we would need to add a lifetime annotation on every reference
    //! in the struct’s definition.

    /// This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds
    /// in its part field.
    pub struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    pub fn struct_holds_reference_with_lifetime() {
        let ie: ImportantExcerpt = ImportantExcerpt { part: "rust" };
        println!("{}", ie.part);
    }
}

pub mod lifetime_elision {
    //! Lifetimes on function or method parameters are called input lifetimes, and lifetimes on
    //! return values are called output lifetimes.
    //!
    //! The compiler uses three rules to figure out the lifetimes of the references when there
    //! are not explicit annotations. The first rule applies to input lifetimes, and the second and
    //! third rules apply to output lifetimes. If the compiler gets to the end of the three rules
    //! and there are still references for which it can’t figure out lifetimes, the compiler will
    //! stop with an error.
    //!
    //! ‼️
    //! The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a
    //! reference. In other words, a function with one parameter gets one lifetime parameter:
    //! `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime
    //! parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.
    //!
    //! ‼️‼️
    //! The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
    //! assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
    //!
    //! ‼️‼️‼️
    //! The third rule is that, if there are multiple input lifetime parameters, but one of them is
    //! `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all
    //! output lifetime parameters.

}

#[cfg(test)]
mod testing {
    #[test]
    fn run_lifetime_annotation_in_struct_struct_holds_reference_with_lifetime() {
        crate::lifetime_annotation_in_struct::struct_holds_reference_with_lifetime();
    }
}
