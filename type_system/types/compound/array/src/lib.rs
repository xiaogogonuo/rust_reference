//! # Array Type
//! An array is a fixed-size sequence of N elements of type T. The array type is written as [T; N].
//! An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
//! Every element of an array must have the same type.
//!
//! # Examples
//!
//! ```
//! let a = [1, 2, 3, 4, 5];
//!
//! let a: [i32; 5] = [1, 2, 3, 4, 5];
//!
//! // initialize an array to contain the same value for each element
//! let a = [3; 5]; // [3, 3, 3, 3, 3]
//!
//! // a stack-allocated array
//! let array: [i32; 3] = [1, 2, 3];
//!
//! // a heap-allocated array
//! let boxed_array: Box<[i32; 3]> = Box::new([1, 2, 3]);
//! ```

/// Access an element by using indexing, `rust` will check that the index you’ve specified is less
/// than the array length. If the index is greater than or equal to the length, `rust` will panic.
pub fn access_array_element() {
    // unsafe mode
    {
        let array: [i8; 5] = [1, 3, 5, 7, 9];
        assert_eq!(array[0], 1);

        let array: [char; 2] = ['中', '_'];
        assert_eq!(array[0], '中');
    }

    // safe mode
    {
        let array: [i8; 5] = [0, 2, 4, 6, 8];
        assert_eq!(array.get(0), Some(&0));
        assert_eq!(array.get(10), None);
        assert_eq!(array.get(0..3), Some(&[0, 2, 4][..]));
        assert_eq!(array.get(0..30), None);
    }
}

mod array_memory_layout {
    pub fn string_array() {
        // every string in array has fixed 24 bytes including a pointer、capacity、len.
        let strings: [String; 2] = ["rust".to_string(), "java".to_string()];

        // • the address of array = the raw pointer to the slice's buffer = the address of first element in array
        // • address between adjacent element is the number of bytes occupied by the String.
        {
            // the address of array
            let string_reference: &[String; 2] = &strings;
            // the raw pointer to the slice's buffer
            let string_raw_pointer: *const String = strings.as_ptr();
            // the address of first element in array
            let first_string_reference: &String = &strings[0];
            // the address of second element in array
            let second_string_reference: &String = &strings[1];
            println!("{:p}", string_reference); // 0x30e75c730
            println!("{:p}", string_raw_pointer); // 0x30e75c730
            println!("{:p}", first_string_reference); // 0x30e75c730
            println!("{:p}", second_string_reference); // 0x30e75c748
        }

        // reference to raw pointer
        /*
         * let v: T = xxx;
         * let v_ref: &T = &v;
         * let v_ptr: *const T = v_ref as *const T;
         */
        {
            let string_reference: &[String; 2] = &strings;
            let first_string_reference: &String = &strings[0];
            println!("{:p}", string_reference as *const String); // 0x30e75c730
            println!("{:p}", first_string_reference as *const String); // 0x30e75c730
        }
    }
}

/// For element in array that its type doesn't implement `Copy` trait, ownership moves to for loop
/// and array can't be used in for loop and afterwards. For element in array that its type
/// implements `Copy` trait, array copied and array can be used in for loop and afterwards.
pub fn iterate_over_array_element() {
    let strings: [String; 2] = ["rust".to_string(), "java".to_string()];
    // move occurs because `strings` has type `[String; 2]`, which does not implement the `Copy` trait
    // `strings` moved due to this for loop implicit call to `.into_iter()`
    for _string in strings {
        // can't invoke strings here
    }
    // can't invoke strings afterwards
    // println!("{:?}", strings);

    let chars: [char; 3] = ['中', '🌞', '!'];
    for _char in chars {
        // can invoke chars here
        chars.len();
    }
    // can invoke chars afterwards
    println!("{:?}", chars);
}

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_array_in_bytes() {
        assert_eq!(std::mem::size_of::<[u8; 3]>(), 3);
        assert_eq!(std::mem::size_of::<[i64; 4]>(), 32);
        assert_eq!(std::mem::size_of::<[String; 2]>(), 48);
    }

    #[test]
    fn run_access_array_element() {
        crate::access_array_element();
    }

    #[test]
    fn run_iterate_over_array_element() {
        crate::iterate_over_array_element();
    }

    #[test]
    fn run_array_memory_layout_string_array() {
        crate::array_memory_layout::string_array();
    }
}
