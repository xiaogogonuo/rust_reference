//! # Lifetimes
//!
//! Lifetimes are another kind of generic which ensure that references are valid. Every reference in
//! Rust has a lifetime, which is the scope for which that reference is valid. Rust requires us to
//! annotate the relationships using generic lifetime parameters to ensure the actual references
//! used at runtime will definitely be valid.
//!
//! The main aim of lifetimes is to prevent dangling references.

pub mod borrow_checker {
    //! The Rust compiler has a borrow checker that compares scopes to determine whether all borrows
    //! are valid.

    pub fn borrow_checker() {
        {
            let x: i8 = 1;     // ----------+-- 'b
                               //           |
            let r: &i8 = &x;   // --+-- 'a  |
                               //   |       |
            println!("{}", r); // --+       |
        }                      // ----------+

        {
            let r: &i8;        // --+-- 'a
            let x: i8 = 1;     // --|-------+-- 'b
            r = &x;            //   |       |
            println!("{}", r); // --+       |
        }                      // ----------+

        // panic case
        // Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
        // At compile time, Rust compares the size of the two lifetimes and sees that r has a
        // lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is
        // rejected because 'b is shorter than 'a: the subject of the reference does not live as
        // long as the reference.
        {
            // let r;                // ---------+-- 'a
            //                       //          |
            // {                     //          |
            //     let x = 5;        // -+-- 'b  |
            //     r = &x;           //  |       |
            // }                     // -+       |
            //                       //          |
            // println!("r: {}", r); // ---------+
            //
        }
    }
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
    //! ‼️ when needs lifetime annotations ‼️
    //! Only when the function's return value contains reference.
    //!
    //! ‼️ how to use lifetime annotations ‼️
    //! To use lifetime annotations in function signatures, we need to declare the generic lifetime
    //! parameters inside angle brackets between the function name and the parameter list.

    pub fn no_need_lifetime_annotation(x: &str, y: &str) -> usize {
        if x.len() > y.len() {
            x.len()
        } else {
            y.len()
        }
    }

    pub fn do_need_lifetime_annotation<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    /// We want the signature to express the following constraint: the returned reference will be
    /// valid as long as both the parameters are valid. This is the relationship between lifetimes
    /// of the parameters and the return value.
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

pub mod understanding_of_valid_reference {
    //! ‼️ how to understand reference `valid` ‼️
    //! The reference parameters are valid means that the value pointed to by reference is valid.

    use super::lifetime_annotation_in_function_signature::longest;

    pub fn right_nth1() {
        let r: &str;                      // -----------------------------------+-- 'o: start
                                          //                                    |
        let s2: String;                   // ------------------+-- 'a: start   |
                                          //                   |               |
        let s1: String;                   // ------------------+-- 'b: start   |
                                          //                   |               |
        s1 = String::from("c++");      //                   |               |
                                          //                   |               |
        {                                 //                   |               |
            s2 = String::from("rust"); //                   |               |
                                          //                   |               |
            r = longest(&s1, &s2);  //                   |               |
        }                                 //                   |               |
                                          //                   |               |
        println!("{}", r);                // ------------------|---------------+ 'o: end
                                          //                   |
    }                                     //-------------------+ 'a 'b: end

    /// Q: The returned reference will be valid as long as both the parameters are valid.
    ///
    /// A: The reference `r1` and `r2` are dropped when the inner scope ends, we can't invoke them
    /// afterward, so `as long as both the parameters are valid` means that as long as the value
    /// the references point to are valid. `s1` and `s2` are valid until the outer scope ends.
    /// so reference `r` refers either `s1` or `s2`, both of `s1` and `s2` live outer of `r`.
    pub fn right_nth2() {
        let r: &str;                              // ----------------------------------+-- 'o: start
                                                  //                                   |
        let s1: String = String::from("rust"); // ------------------+-- 'a: start   |
                                                  //                   |               |
        let s2: String = String::from("c++");  // ------------------+-- 'b: start   |
                                                  //                   |               |
        {                                         //                   |               |
            let r1: &str = s1.as_str();           // --+-- 'c: start   |               |
                                                  //   |               |               |
            let r2: &str = s2.as_str();           // --+-- 'd: start   |               |
                                                  //   |               |               |
            r = longest(r1, r2);            // --+ 'c 'd: end    |               |
        }                                         //                   |               |
                                                  //                   |               |
        println!("{}", r);                        // ------------------|---------------+ 'o: end
                                                  //                   |
    }                                             // ------------------+ 'a 'b: end

    pub fn right_nth3() {
        let r: &str;                                 // ---------------------------------------------+-- 'o: start
                                                     //                                              |
        let s1: String = String::from("rust");    // -------------------------------+-- 'a: start |
                                                     //                                |             |
        {                                            //                                |             |
            let s2: String = String::from("c++"); // -----------------+-- 'b: start |             |
                                                     //                  |             |             |
            let r1: &str = s1.as_str();              // --+-- 'c: start  |             |             |
                                                     //   |              |             |             |
            let r2: &str = s2.as_str();              // --+-- 'd: start  |             |             |
                                                     //   |              |             |             |
            r = longest(r1, r2);               // --+ 'c 'd: end   |             |             |
                                                     //                  |             |             |
            println!("{}", r);                       // -----------------|-------------|-------------+ 'o: end
                                                     //                  |             |
        }                                            // -----------------+ 'b: end     |
                                                     //                                |
    }                                                // -------------------------------+ 'a: end

    pub fn error_nth1() {
        let r: &str;                                 // ---------------------------------------------+-- 'o: start
                                                     //                                              |
        let s1: String = String::from("rust");    // -------------------------------+-- 'a: start |
                                                     //                                |             |
        {                                            //                                |             |
            let s2: String = String::from("c++"); // -----------------+-- 'b: start |             |
                                                     //                  |             |             |
            let r1: &str = s1.as_str();              // --+-- 'c: start  |             |             |
                                                     //   |              |             |             |
            let r2: &str = s2.as_str();              // --+-- 'd: start  |             |             |
                                                     //   |              |             |             |
            r = longest(r1, r2);               // --+ 'c 'd: end   |             |             |
        }                                            // -----------------+ 'b: end     |             |
                                                     //                                |             |
        // `s2` does not live long enough            //                                |             |
        // println!("{}", r);                        // -------------------------------|-------------+ 'o: end
                                                     //                                |
    }                                                // -------------------------------+ 'a: end
}

pub mod thinking_in_terms_of_lifetime {

    /// If we changed the implementation of the `longest` function to always return the first
    /// parameter rather than the longest string slice, we do not need to specify a lifetime on the
    /// `y` parameter. The following code will compile.
    pub fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }
}

pub mod lifetime_annotation_in_struct_definitions {
    //! So far, the structs we’ve defined all hold owned types. We can define structs to hold
    //! references, but we would need to add a lifetime annotation on every reference in the
    //! struct’s definition.

    /// This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds
    /// in its part field.
    pub struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    pub fn right_nth1() {
        let novel: String = String::from("rust will be the best language in the future.");
        let ie: ImportantExcerpt = ImportantExcerpt { part: &novel[..4] };
        println!("{}", ie.part);
    }

    pub fn error_nth1() {
        let ie: ImportantExcerpt;
        {
            let novel: String = String::from("rust will be the best language in the future.");
            ie = ImportantExcerpt { part: &novel[..4] };
        }
        // error[E0597]: `novel` does not live long enough
        // println!("{}", ie.part);
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
    //! ‼️-‼️
    //! The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
    //! assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
    //!
    //! ‼️-‼️-‼️
    //! The third rule is that, if there are multiple input lifetime parameters, but one of them is
    //! `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all
    //! output lifetime parameters.
}

pub mod lifetime_annotation_in_method {
    //! Lifetime names for struct fields always need to be declared after the impl keyword and then
    //! used after the struct’s name, because those lifetimes are part of the struct’s type.
    //!
    //! In method signatures inside the impl block, references might be tied to the lifetime of
    //! references in the struct’s fields, or they might be independent. In addition, the lifetime
    //! elision rules often make it so that lifetime annotations are not necessary in method
    //! signatures.

    pub struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        /// A method named level whose only parameter is a reference to self and whose return value
        /// is an `i32`, which is not a reference to anything.
        pub fn level(&self) -> i32 {
            0
        }

        pub fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

pub mod static_lifetime {
    //! One special lifetime is `'static`, which denotes that the affected reference can live for
    //! the entire duration of the program. All string literals have the `'static` lifetime, which
    //! we can annotate as follows:
    //! ```let s: &'static str = "I have a static lifetime.";```
    //!
    //! This string is stored directly in the program’s binary, which is always available.
    //! Therefore, the lifetime of all string literals is `'static`.
}

pub mod generic_type_trait_bound_lifetime {
    pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("{}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
