//! # Function
//!
//! A functions consists of a block, along with a name, a set of parameters, and an output type.
//! Functions are declared with the keyword `fn`. If the output type is not explicitly stated, it
//! is the unit type.
//!

#[allow(dead_code)]
fn function_parameter((value, _): (i32, i32)) -> i32 {
    value
}

#[allow(dead_code)]
fn function_body() {
    // Statements
    // Statements do not return values. You can’t assign a let statement to another variable like:
    // let x = (let y = 6);

    // Expressions
    // • A math operation is an expression
    // • Calling a functions is an expression
    // • Calling a macro is an expression
    // • A new scope block created with curly brackets is an expression
    let _y = {
        let x = 3;
        x + 1
    };
}

#[allow(dead_code)]
fn function_return() -> () {}
