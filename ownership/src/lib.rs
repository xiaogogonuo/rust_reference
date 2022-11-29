//! # Ownership
//!
//! # Stack
//! The stack stores values in the order it gets them and removes the values in the opposite order.
//! This is referred to as last in, first out. Adding data is called pushing onto the stack, and
//! removing data is called popping off the stack. All data stored on the stack must have a known,
//! fixed size.
//!
//! # Heap
//! Data with an unknown size at compile time or a size that might change must be stored on the heap.
//! When you put data on the heap, you request a certain amount of space. The memory allocator finds
//! an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer,
//! which is the address of that location. This process is called allocating on the heap and is
//! sometimes abbreviated as just allocating(pushing values onto the stack is not considered
//! allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer
//! on the stack, but when you want the actual data, you must follow the pointer.
//!
//! # Stack vs Heap
//! Pushing to the stack is faster than allocating on the heap because the allocator never has to
//! search for a place to store new data; that location is always at the top of the stack.
//! Accessing data in the heap is slower than accessing data on the stack because you have to follow
//! a pointer to get there.
//!
//! # Apply
//! When your code calls a functions, the values passed into the functions (including, potentially,
//! pointers to data on the heap) and the functions’s local variables get pushed onto the stack.
//! When the functions is over, those values get popped off the stack.
//!
//! # Ownership Rules
//! * Each value in Rust has an owner
//! * There can only be one owner at a time
//! * When the owner goes out of scope, the value will be dropped

mod variable_scope {
    //! A scope is the range within a program for which an item is valid
    //!
    //! ```
    //!     {                      // s is not valid here, it’s not yet declared
    //!         let s = "cargo";   // s is valid from this point forward
    //!
    //!         // do stuff with s
    //!     }                      // this scope is now over, and s is no longer valid
    //! ```
}

mod ownership {

    pub mod with_copy {
        //! Rust has a special annotation called the `Copy` trait that we can place on types that
        //! are stored on the stack. If a type implements the `Copy` trait, variables that use it
        //! do not move, but rather are trivially copied, making them still valid after assignment
        //! to another variable.
        //!
        //! Types implement the `Copy` trait
        //! * All the integer types
        //! * The Boolean type
        //! * All the floating point types
        //! * The character type
        //! * Tuples, if they only contain types that also implement Copy. For example, (i32, i32)
        //! implements Copy, but (i32, String) does not.
        //! * Arrays, if they only contain types that also implement Copy. For example, [i32; 2]
        //! implements Copy, but [String; 2] does not.
        //! * &T, but &mut T does not

        /// Integers are simple values with a known, fixed size, and these two 5 values are pushed
        /// onto the stack.
        pub fn multiple_variables_interact_with_the_same_data() {
            let x: i32 = 5; // bind the value 5 to x
            let y: i32 = x; // make a copy of the value in x and bind it to y
            println!("{}{}", x, y);
        }
    }

    pub mod with_move {
        //! # Ownership and Functions
        //!
        //! # Examples
        //!
        //! ```
        //! fn main() {
        //!     let s = String::from("hello");  // s comes into scope
        //!
        //!     takes_ownership(s);             // s's value moves into the functions...
        //!                                     // ... and so is no longer valid here
        //!
        //!     let x = 5;                      // x comes into scope
        //!
        //!     makes_copy(x);                  // x would move into the functions,
        //!                                     // but i32 is Copy, so it's okay to still
        //!                                     // use x afterward
        //!
        //! } // Here, x goes out of scope, then s. But because s's value was moved, nothing
        //!   // special happens.
        //!
        //! fn takes_ownership(some_string: String) { // some_string comes into scope
        //!     println!("{}", some_string);
        //! } // Here, some_string goes out of scope and `drop` is called. The backing
        //!   // memory is freed.
        //!
        //! fn makes_copy(some_integer: i32) { // some_integer comes into scope
        //!     println!("{}", some_integer);
        //! } // Here, some_integer goes out of scope. Nothing special happens.
        //! ```
        //!
        //! # Return Values and Scope
        //!
        //! # Examples
        //! ```
        //! fn main() {
        //!     let s1 = gives_ownership();         // gives_ownership moves its return
        //!                                         // value into s1
        //!
        //!     let s2 = String::from("hello");     // s2 comes into scope
        //!
        //!     let s3 = takes_and_gives_back(s2);  // s2 is moved into
        //!                                         // takes_and_gives_back, which also
        //!                                         // moves its return value into s3
        //! } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
        //!   // happens. s1 goes out of scope and is dropped.
        //!
        //! fn gives_ownership() -> String {             // gives_ownership will move its
        //!                                              // return value into the functions
        //!                                              // that calls it
        //!
        //!     let some_string = String::from("yours"); // some_string comes into scope
        //!
        //!     some_string                              // some_string is returned and
        //!                                              // moves out to the calling
        //!                                              // functions
        //! }
        //!
        //! // This functions takes a String and returns one
        //! fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
        //!                                                       // scope
        //!
        //!     a_string  // a_string is returned and moves out to the calling functions
        //! }
        //! ```

        pub fn multiple_variables_interact() {
            let s1: String = String::from("rust");
            let s2: String = s1; // s1 is no longer valid.
                                 // Rust does not need to free anything when s1 goes out of scope.
            println!("{}", s2);
        }

        pub fn deeply_copy_heap_data() {
            let s1 = String::from("hello");
            let s2 = s1.clone();

            println!("s1 = {}, s2 = {}", s1, s2);
        }
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_ownership_with_move_multiple_variables_interact() {
        crate::ownership::with_move::multiple_variables_interact();
    }

    fn run_ownership_with_move_deeply_copy_heap_data() {
        crate::ownership::with_move::deeply_copy_heap_data();
    }
}
