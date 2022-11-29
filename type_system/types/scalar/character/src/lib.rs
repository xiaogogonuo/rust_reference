//! # Character Type
//! Specify `char` literals with single quotes, `char` type in `rust` is four bytes in size and
//! represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
//! Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all
//! valid char values in `rust`.
//!
//! # Examples
//!
//! ```
//! let c = 'z';
//! let z: char = 'ℤ'; // with explicit type annotation
//! let heart_eyed_cat = '😻';
//! ```

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_char_in_bytes() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }
}
