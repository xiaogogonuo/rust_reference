//! # Integer Types
//! ```text
//! Length      Signed      Unsigned
//! 8-bit       i8          u8
//! 16-bit      i16         u16
//! 32-bit      i32         u32
//! 64-bit      i64         u64
//! 128-bit     i128        u128
//! arch        isize       usize
//! ```
//!
//! # Forms of integer literals
//! ```text
//! Number literals     Example
//! Decimal             98_222
//! Hex                 0xff
//! Octal               0o77
//! Binary              0b1111_0000
//! Byte(u8 only)       b'A'
//! ```
//!
//! # Machine dependent integer types
//! The `usize` type is an unsigned integer type with the same number of bits as the platform's
//! pointer type. It can represent every memory address in the process. The `isize` type is a signed
//! integer type with the same number of bits as the platform's pointer type. The theoretical upper
//! bound on object and array size is the maximum isize value.
//!
//! # Integer Overflow
//! When compiling in debug mode, `rust` checks for integer overflow that cause panics. When
//! compiling in release mode, `rust` doesn't check for integer overflow that cause panics.

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_integer_in_bytes() {
        assert_eq!(std::mem::size_of::<u8>(), 1);
        assert_eq!(std::mem::size_of::<i8>(), 1);
        assert_eq!(std::mem::size_of::<u16>(), 2);
        assert_eq!(std::mem::size_of::<i16>(), 2);
        assert_eq!(std::mem::size_of::<u32>(), 4);
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<u64>(), 8);
        assert_eq!(std::mem::size_of::<i64>(), 8);
        assert_eq!(std::mem::size_of::<u128>(), 16);
        assert_eq!(std::mem::size_of::<i128>(), 16);
        assert_eq!(std::mem::size_of::<usize>(), 8);
        assert_eq!(std::mem::size_of::<isize>(), 8);
    }
}
