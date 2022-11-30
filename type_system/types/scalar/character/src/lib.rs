//! # Character
//!
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

pub mod character_attribute {
    /// Returns the number of bytes this char would need if encoded in UTF-8.
    pub fn len_utf8() {
        assert_eq!('x'.len_utf8(), 1);
        assert_eq!('£'.len_utf8(), 2);
        assert_eq!('中'.len_utf8(), 3);
        assert_eq!('🔥'.len_utf8(), 4);
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_char_in_bytes() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn run_character_attribute_len_utf8() {
        crate::character_attribute::len_utf8();
    }
}
