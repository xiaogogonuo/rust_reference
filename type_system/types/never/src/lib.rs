//! # Never Type
//! The never type `!` is a type with no values, representing the result of computations that never
//! complete. Expressions of type ! can be coerced into any other type.

pub fn never_return() -> ! {
    loop {}
}

#[cfg(test)]
mod testing {
    #[test]
    fn coerce() {
        crate::never_return();
    }
}
