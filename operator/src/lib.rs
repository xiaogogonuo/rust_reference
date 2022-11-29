//! # Numeric Operations
//! `rust` supports the basic mathematical operations you’d expect for all of the number types:
//! addition, subtraction, multiplication, division, and remainder. Integer division rounds down to
//! the nearest integer.

#[cfg(test)]
mod testing {
    #[test]
    fn numeric_operations() {
        // addition
        assert_eq!(1 + 2, 3);

        // subtraction
        assert_eq!(95.5 - 4.3, 91.2);

        // division
        assert_eq!(2 / 3, 0);
        assert_eq!(-5 / 4, -0);

        // remainder
        assert_eq!(43 % 5, 3);
    }
}

// TODO
// https://doc.rust-lang.org/stable/book/appendix-02-operators.html
