/// variable in `rust` is immutable default
pub fn immutable_variable() {
    let x = 1;

    // can't assign twice to immutable variable
    // x = 2;
}

/// make variable mutable by adding `mut` in front of the variable name
pub fn mutable_variable() {
    let mut x = 1;
    x = 2;
}

/// unused variable causes warnings at compile time, prefix it with an underscore to ignore warning
pub fn unused_variable() {
    let _x = 1;
}

/// variable without type annotation in `rust` has default type
/// * `i32` for int
/// * `f64` for float
///
/// add a type annotation after variable name to specify it's type
pub fn type_annotation_for_variable() {
    let _i: u8 = 1;
    let _f: f32 = 1.2;
}

/// * the type of the value must be annotated
/// * not allowed to use `mut` with constants
pub const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;

pub mod scope {
    pub const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /// * variable can only be declared in functions
    pub fn variable_scope() {
        let two_minutes_in_seconds = 60 * 2;

        {
            let one_minutes_in_seconds = 60 * 1;
            println!("{}", one_minutes_in_seconds);
            println!("{}", two_minutes_in_seconds);
            println!("{}", crate::FOUR_HOURS_IN_SECONDS);
            println!("{}", THREE_HOURS_IN_SECONDS);
        }

        println!("{}", two_minutes_in_seconds);
        println!("{}", crate::FOUR_HOURS_IN_SECONDS);
        println!("{}", THREE_HOURS_IN_SECONDS);
    }

    /// * constants can be declared in any scopes
    pub fn constant_scope() {
        const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;

        {
            const ONE_HOURS_IN_SECONDS: u32 = 60 * 60 * 1;
            println!("{}", crate::FOUR_HOURS_IN_SECONDS);
            println!("{}", THREE_HOURS_IN_SECONDS);
            println!("{}", TWO_HOURS_IN_SECONDS);
            println!("{}", ONE_HOURS_IN_SECONDS);
        }

        println!("{}", crate::FOUR_HOURS_IN_SECONDS);
        println!("{}", THREE_HOURS_IN_SECONDS);
        println!("{}", TWO_HOURS_IN_SECONDS);
    }
}

/// shadowing can change the type of the value but reuse the same name
pub fn variable_shadowing() {
    let x = 5;
    let x = x + 1; // shadowing x once but not change the type
    {
        let x = x * 2; // shadowing x again but also not change the type
        assert_eq!(x, 12);
    }
    assert_eq!(x, 6);

    let spaces = "    ";
    let spaces = spaces.len();
    assert_eq!(spaces, 4);

    let s = String::from("\t\n 100 \t\n");
    let s: i32 = s.trim().parse().unwrap();
    assert_eq!(s, 100);
}
