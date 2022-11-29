//! # Boolean type
//! The boolean type can take on one of two values, `true` and `false`.
//!
//! The boolean type is the type of many operands in various expressions:
//! * The condition operand in if expressions and while expressions
//! * The operands in lazy boolean operator expressions

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_boolean_in_bytes() {
        assert_eq!(std::mem::size_of::<bool>(), 1);
        assert_eq!(true, 0x01);
    }
}
