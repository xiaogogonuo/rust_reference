//! # String
//!
//! Rust has only one string type in the core language, which is the string slice `str` that is
//! usually seen in its borrowed form `&str`.
//!
//! The `String` type, which is provided by Rust’s standard library rather than coded into the core
//! language, is a growable, mutable, owned, UTF-8 encoded string type.
//!
//! A `String` is made up of three components: a pointer to some bytes, a length, and a capacity.
//! The pointer points to an internal buffer `String` uses to store its data. The length is the
//! number of bytes currently stored in the buffer, the capacity is the size of the buffer in bytes.

/// A `String` 0~8 bytes store the underline data pointer, 8~16 bytes store capacity, 16~24 bytes
/// store length.
/// ```text
/// -------------- 0x3053bd718
/// 0x7f9f34804080
/// -------------- 0x3053bd720
///      500
/// -------------- 0x3053bd728
///       4
/// -------------- 0x3053bd730
/// ```
pub fn string_memory_layout() {
    let mut s: String = String::with_capacity(500);
    s.push_str("rust");
    let p: *const String = &s;
    println!("string address: {:p}", p); // string address: 0x3053bd718
    println!("underline data address: {:p}", s.as_ptr()); // underline data address: 0x7f9f34804080
    unsafe {
        println!(
            "0~8 bytes store underline pointer: {:#0x?}",
            *(p as *const u64)
        ); // 0~8 bytes store underline pointer: 0x7f9f34804080

        let capacity_address: *const u64 = (p as usize + 8) as *const u64;
        println!(
            "8~16 bytes store capacity: {:p}:{}",
            capacity_address, *capacity_address
        ); // 8~16 bytes store capacity: 0x3053bd720:500

        let length_address: *const u64 = (p as usize + 16) as *const u64;
        println!(
            "16~24 bytes store length: {:p}:{}",
            length_address, *length_address
        ); // 16~24 bytes store length: 0x3053bd728:4
    }
}

pub mod create_string {
    /// Creates a new empty String.
    pub fn with_new() {
        let _immutable_string: String = String::new();
        assert_eq!(_immutable_string.len(), 0);
        assert_eq!(_immutable_string.capacity(), 0);

        let mut _mutable_string: String = String::new();
        assert_eq!(_mutable_string.len(), 0);
        assert_eq!(_mutable_string.capacity(), 0);
    }

    /// Converts a &str into a [String].
    pub fn with_from() {
        let _immutable_string: String = String::from("rust");
        assert_eq!(_immutable_string.len(), 4);
        assert_eq!(_immutable_string.capacity(), 4);

        let mut _mutable_string: String = String::from("rust");
        assert_eq!(_mutable_string.len(), 4);
        assert_eq!(_mutable_string.capacity(), 4);
    }

    /// Creates a new empty String with at least the specified capacity.
    pub fn with_capacity() {
        let _immutable_string: String = String::with_capacity(5);
        assert_eq!(_immutable_string.len(), 0);
        assert_eq!(_immutable_string.capacity(), 5);

        let mut _mutable_string: String = String::with_capacity(5);
        assert_eq!(_mutable_string.len(), 0);
        assert_eq!(_mutable_string.capacity(), 5);
    }

    /// Uses the `to_string` method, which is available on any type that implements the `Display`
    /// trait, as string literals do.
    pub fn to_string() {
        let _immutable_string: String = "rust".to_string();
        assert_eq!(_immutable_string.len(), 4);
        assert_eq!(_immutable_string.capacity(), 4);
    }
}

pub mod update_string {

    /// Combines two existing strings with `+` operator.
    ///
    /// The `+` operator uses the `add` method, whose signature looks something like this:
    /// `fn add(mut self, other: &str) -> String`
    ///
    /// The type of `&s2` is `&String`, not `&str`, as specified in the second parameter to add.
    /// So why does it compile? The reason we’re able to use `&s2` in the call to add is that the
    /// compiler can coerce the `&String` argument into a `&str`. When we call the add method,
    /// Rust uses a deref coercion, which here turns `&s2` into `&s2[..]`.
    pub fn with_plus_operator() {
        // ---- testing::run_update_string_with_plus_operator stdout ----
        // [src/lib.rs:114] s1.as_ptr() = 0x0000600000a7c030
        // [src/lib.rs:115] s1.len() = 7
        // [src/lib.rs:116] s1.capacity() = 7
        // [src/lib.rs:118] s2.as_ptr() = 0x0000600000a7c020
        // [src/lib.rs:120] _s3.as_ptr() = 0x0000600000a7c030
        // [src/lib.rs:121] _s3.len() = 12
        // [src/lib.rs:122] _s3.capacity() = 14
        let s1: String = String::from("hello, ");
        dbg!(s1.as_ptr());
        dbg!(s1.len());
        dbg!(s1.capacity());
        let s2: String = String::from("rust!");
        dbg!(s2.as_ptr());
        let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        dbg!(_s3.as_ptr());
        dbg!(_s3.len());
        dbg!(_s3.capacity());

        // concatenate multiple strings
        let s1: String = String::from("tic");
        let s2: String = String::from("tac");
        let s3: String = String::from("toe");
        let _s = s1 + "-" + &s2 + "-" + &s3;
    }

    /// Creates a String using interpolation of runtime expressions.
    ///
    /// `format!` macro uses references so that this call does not take ownership of any of its
    /// parameters.
    pub fn with_format_marco() {
        let s1: String = String::from("tic");
        let s2: String = String::from("tac");
        let s3: String = String::from("toe");
        let _s = format!("{}-{}-{}", s1, s2, s3);
    }

    /// Appends a given string slice onto the end of this String.
    pub fn with_push_str() {
        // ---- testing::run_update_string_with_push_str stdout ----
        // [src/lib.rs:109] s.len() = 0
        // [src/lib.rs:110] s.capacity() = 0
        // [src/lib.rs:111] s.as_ptr() = 0x0000000000000001
        // [src/lib.rs:113] s.len() = 4
        // [src/lib.rs:114] s.capacity() = 8
        // [src/lib.rs:115] s.as_ptr() = 0x0000600003df8000
        // [src/lib.rs:121] s == "rust" = true
        let mut s: String = "".to_string();
        dbg!(s.len());
        dbg!(s.capacity());
        dbg!(s.as_ptr());
        s.push_str("rust");
        dbg!(s.len());
        dbg!(s.capacity());
        dbg!(s.as_ptr());
        dbg!(s == "rust"); // String can compare with &str

        // [src/lib.rs:120] s.len() = 0
        // [src/lib.rs:121] s.capacity() = 5
        // [src/lib.rs:122] s.as_ptr() = 0x0000600001a64010
        // [src/lib.rs:124] s.len() = 4
        // [src/lib.rs:125] s.capacity() = 5
        // [src/lib.rs:126] s.as_ptr() = 0x0000600001a64010
        // [src/lib.rs:127] s == "rust" = true
        let mut s: String = String::with_capacity(5);
        dbg!(s.len());
        dbg!(s.capacity());
        dbg!(s.as_ptr());
        s.push_str("rust");
        dbg!(s.len());
        dbg!(s.capacity());
        dbg!(s.as_ptr());
        dbg!(s == "rust"); // String can compare with &str
    }

    /// Appends the given [char] to the end of this String.
    pub fn with_push() {
        let mut s: String = "".to_string();
        s.push('r');
    }
}

pub mod index_string {
    //! Rust strings don’t support indexing.
    //!
    //! If you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
    //! ```panic
    //! let s1 = String::from("hello");
    //! let h = s1[0];
    //! ```
    //!
    //! A `String` is a wrapper over a `Vec<u8>`.

    /// Examples of UTF-8 encoding
    ///
    /// ```text
    /// Character           𐍈
    /// Unicode             U+10348
    /// Binary code point   0 0001 0000 0011 0100 1000
    /// Binary UTF-8        11110000 10010000 10001101 10001000
    /// Hex UTF-8           F0 90 8D 88
    /// ```
    ///
    /// 0xF0 = 240
    /// 0x90 = 144
    /// 0x8D = 141
    /// 0x88 = 136
    ///
    /// `𐍈` with UTF-8 encoding allocates 4 bytes
    pub fn internal_representation() {
        // ---- testing::run_read_string_internal_representation stdout ----
        // [src/lib.rs:211] s.len() = 4
        // [src/lib.rs:209] s.as_bytes() = [240, 144, 141, 136]
        let s: String = "𐍈".to_string();
        dbg!(s.len());
        dbg!(s.as_bytes());
    }
}

pub mod slice_string {
    //! Rather than indexing using `[]` with a single number, you can use `[]` with a range to
    //! create a string slice containing particular bytes.

    pub fn with_range() {
        // Here, _s will be a &str that contains the first 4 bytes of the string. Earlier, we
        // mentioned that each of these characters was 2 bytes, which means _s will be Зд.
        let hello: String = "Здравствуйте".to_string();
        let _s: &str = &hello[0..4];
    }
}

pub mod iter_string {
    pub fn with_chars() {
        // ---- testing::run_iter_string_with_chars stdout ----
        // 中
        // 国
        let zh: String = "中国".to_string();
        for c in zh.chars() {
            println!("{}", c);
        }

        // also be suitable for &str
        dbg!("中国".chars());
    }

    pub fn with_bytes() {
        // ---- testing::run_iter_string_with_bytes stdout ----
        // 228
        // 184
        // 173
        // 229
        // 155
        // 189
        let zh: String = "中国".to_string();
        for c in zh.bytes() {
            println!("{}", c);
        }

        // also be suitable for &str
        dbg!("中国".bytes());
    }
}

pub mod string_attribute {
    use std::ops::Index;

    /// Returns the length of this String, in bytes, not [char]s or graphemes.
    pub fn len() {
        // One ascii character occupies 1 bytes
        assert_eq!(String::from("z").len(), 1);

        // A chinese character occupies 3 bytes
        assert_eq!(String::from("中").len(), 3);

        // A emotion character occupies 4 bytes
        assert_eq!(String::from("🔥").len(), 4);

        assert_eq!(String::from("z中🔥").len(), 8)
    }

    pub fn contains() {
        // ---- testing::run_string_attribute_contains stdout ----
        // [src/lib.rs:285] String::from("rust").contains("u") = true
        dbg!(String::from("rust").contains("u"));
    }

    /// Replaces all matches of a pattern with another string.
    pub fn replace() {
        // ---- testing::run_string_attribute_replace stdout ----
        // [src/lib.rs:295] s = "r u s t"
        // [src/lib.rs:296] n = "rust"
        let s: String = "r u s t".to_string();
        let n = s.replace(" ", "");
        dbg!(s);
        dbg!(n);
    }

    /// Replaces first N matches of a pattern with another string.
    pub fn replacen() {
        // ---- testing::run_string_attribute_replacen stdout ----
        // [src/lib.rs:304] s = "r u s t"
        // [src/lib.rs:305] n = "rus t"
        let s: String = "r u s t".to_string();
        let n = s.replacen(" ", "", 2);
        dbg!(s);
        dbg!(n);
    }

    /// Removes a [char] from this String at a byte position and returns it.
    pub fn remove() {
        // ---- testing::run_string_attribute_remove stdout ----
        // [src/lib.rs:303] s = "ru s t"
        // [src/lib.rs:304] removed_char = ' '
        let mut s: String = "r u s t".to_string();
        let removed_char: char = s.remove(1);
        dbg!(s);
        dbg!(removed_char);
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_string_in_bytes() {
        assert_eq!(std::mem::size_of::<String>(), 24);
    }

    #[test]
    fn run_string_memory_layout() {
        crate::string_memory_layout();
    }

    #[test]
    fn run_create_string_with_new() {
        crate::create_string::with_new();
    }

    #[test]
    fn run_create_string_with_from() {
        crate::create_string::with_from();
    }

    #[test]
    fn run_create_string_with_capacity() {
        crate::create_string::with_capacity();
    }

    #[test]
    fn run_create_string_to_string() {
        crate::create_string::to_string();
    }

    #[test]
    fn run_update_string_with_plus_operator() {
        crate::update_string::with_plus_operator();
    }

    #[test]
    fn run_update_string_with_format_marco() {
        crate::update_string::with_format_marco();
    }

    #[test]
    fn run_update_string_with_push_str() {
        crate::update_string::with_push_str();
    }

    #[test]
    fn run_update_string_with_push() {
        crate::update_string::with_push();
    }

    #[test]
    fn run_index_string_internal_representation() {
        crate::index_string::internal_representation();
    }

    #[test]
    fn run_slice_string_with_range() {
        crate::slice_string::with_range();
    }

    #[test]
    fn run_iter_string_with_chars() {
        crate::iter_string::with_chars();
    }

    #[test]
    fn run_iter_string_with_bytes() {
        crate::iter_string::with_bytes();
    }

    #[test]
    fn run_string_attribute_len() {
        crate::string_attribute::len();
    }

    #[test]
    fn run_string_attribute_contains() {
        crate::string_attribute::contains();
    }

    #[test]
    fn run_string_attribute_replace() {
        crate::string_attribute::replace();
    }

    #[test]
    fn run_string_attribute_replacen() {
        crate::string_attribute::replacen();
    }

    #[test]
    fn run_string_attribute_remove() {
        crate::string_attribute::remove();
    }
}
