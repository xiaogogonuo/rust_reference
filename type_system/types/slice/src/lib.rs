//! # Slice type
//! A slice is a dynamically sized type representing a 'view' into a sequence of elements of type T.
//! The slice type is written as [T]. It doesn't have a size known at compile-time, so slice type
//! generally used through pointer types:
//! * &[T]: a 'shared slice', it doesn't own the data it points to; it borrows it.
//! * &mut [T]: a 'mutable slice'. It mutably borrows the data it points to.
//! * Box<[T]>: a 'boxed slice'.
//!
//! A slice is a kind of reference, so it does not have ownership.
//! A slice stores a reference to the first element and a length.
//!
//! # Examples
//!
//! ```
//! // A heap-allocated array, coerced to a slice
//! let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
//!
//! // A (shared) slice into an array
//! let slice: &[i32] = &boxed_array[..];
//! ```

mod string_slice {
    //! A string slice is a reference to part of a `String`
    //!
    //! `String` slice range indices must occur at valid UTF-8 character boundaries. If you attempt
    //! to create a string slice in the middle of a multibyte character, your program will exit with
    //! an error.

    #[allow(dead_code)]
    pub fn memory_layout() {
        let s: String = String::from("rust和cargo");
        let rust: &str = &s[0..4];
        let cargo: &str = &s[7..12];
        println!("{:p}", s.as_ptr()); // 0x600002fc0000
        println!("{:p}", rust); // 0x600002fc0000
        println!("{:p}", cargo); // 0x600002fc0007
    }

    #[allow(dead_code)]
    pub fn string_literal() {
        // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
        // This is also why string literals are immutable; &str is an immutable reference.
        let s: &str = "rust";
        let from_slice: &str = &s[..2];
        assert_eq!(from_slice, "ru");
    }

    #[allow(dead_code)]
    pub fn string_slice_as_parameter(s: &str) -> &str {
        &s[..]
    }
}

#[allow(dead_code)]
fn first_word_index(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();
    // usage 1
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // usage 2
    // for (i, item) in bytes.iter().enumerate() {
    //     if *item == b' ' {
    //         return i;
    //     }
    // }

    s.len()
}

#[allow(dead_code)]
fn first_word_slice(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

mod array_slice {
    #[allow(dead_code)]
    pub fn builder() {
        let array: [i8; 5] = [1, 2, 3, 4, 5];
        let slice: &[i8] = &array[..3];
        assert_eq!(slice, &[1, 2, 3]);
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_string_slice_memory_layout() {
        crate::string_slice::memory_layout();
    }

    #[test]
    fn run_first_word_index() {
        assert_eq!(crate::first_word_index(&"rust".to_string()), 4);
        assert_eq!(crate::first_word_index(&"ru t".to_string()), 2);
    }

    #[test]
    fn run_first_word_slice() {
        assert_eq!(crate::first_word_slice(&"rust".to_string()), "rust");
        assert_eq!(crate::first_word_slice(&"中国 美国".to_string()), "中国");
    }

    #[test]
    fn run_string_slice_as_parameter() {
        assert_eq!(
            crate::string_slice::string_slice_as_parameter("rust"),
            "rust"
        );
        assert_eq!(
            crate::string_slice::string_slice_as_parameter(&"rust".to_string()),
            "rust"
        );
        assert_eq!(
            crate::string_slice::string_slice_as_parameter(&"rust".to_string()[..]),
            "rust"
        );
    }

    #[test]
    fn run_array_slice_builder() {
        crate::array_slice::builder();
    }
}
