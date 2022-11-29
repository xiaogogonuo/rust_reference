//! # Tuple Type
//! Tuples group values with a variety of types into one compound type. Tuples have a fixed length.
//!
//! # Examples
//!
//! ```
//! let tup: (i32, f64, u8) = (500, 6.4, 1);
//! ```
//!
//! Some examples of tuple types:
//! * ()
//! * (f64, f64)
//! * (String, i32)
//! * (i32, String) (different type from the previous)
//! * (i32, f64, Vec<String>, Option<bool>)
//!
//! The tuple without any values has a special name, unit. This value and its corresponding type are
//! both written () and represent an empty value or an empty return type. Expressions implicitly
//! return the unit value if they don’t return any other value.

pub fn access_tuple_element() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;
}

pub fn destructure_tuple() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

}
