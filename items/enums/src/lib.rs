////////////////////////////////////////////////////////////////////////////////
// Defining Enum
////////////////////////////////////////////////////////////////////////////////
mod defining_enum {
    #[allow(dead_code)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[allow(dead_code)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        Color(i32, i32, i32),
    }

    #[allow(dead_code)]
    fn instantiating_enums() {
        {
            let _v4 = IpAddrKind::V4;
            let _v6 = IpAddrKind::V6;
        }

        {
            let _v4 = IpAddr::V4(127, 0, 0, 1);
            let _v6 = IpAddr::V6(String::from("::1"));
        }

        {
            let _quit = Message::Quit;
            let _move = Message::Move { x: 1, y: 2 };
            let _write = Message::Write(String::from("rust"));
            let _color = Message::Color(0, 0, 0);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Option Enum
////////////////////////////////////////////////////////////////////////////////
mod option_enum {
    //! ```
    //! enum Option<T> {
    //!     None,
    //!     Some(T),
    //! }
    //! ```
    //!
    //! The type of some_number is `Option<i32>`. The type of some_char is `Option<char>`, which is
    //! a different type. Rust can infer these types because we’ve specified a value inside the Some
    //! variant. For absent_number, Rust requires us to annotate the overall `Option` type: the
    //! compiler can’t infer the type that the corresponding `Some` variant will hold by looking
    //! only at a `None` value. Here, we tell Rust that we mean for absent_number to be of type
    //! `Option<i32>`.
    //! ```
    //! let some_number = Some(5);
    //! let some_char = Some('e');
    //!
    //! let absent_number: Option<i32> = None;
    //! ```
}

////////////////////////////////////////////////////////////////////////////////
// Patterns Bind To Values
////////////////////////////////////////////////////////////////////////////////
mod patterns_bind_to_values {
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum UsState {
        Alaska,
        Alabama,
    }

    #[allow(dead_code)]
    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[allow(dead_code)]
    pub fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // State quarter from Alaska!
                // State quarter from Alabama!
                println!("State quarter from {:?}!", state);
                match state {
                    UsState::Alaska => 25,
                    UsState::Alabama => 26,
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matching With Option
////////////////////////////////////////////////////////////////////////////////
mod matching_with_option {
    #[allow(dead_code)]
    pub fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Catch All Patterns
////////////////////////////////////////////////////////////////////////////////
mod catch_all_patterns {
    //! Note that we have to put the catch-all arm last because the patterns are evaluated in order.
    //! If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if
    //! we add arms after a catch-all.
    //!
    //! Rust also has a pattern we can use when we want a catch-all but don’t want to use the value
    //! in the catch-all pattern: `_` is a special pattern that matches any value and does not bind
    //! to that value. This tells Rust we are not going to use the value, so Rust won’t warn us about
    //! an unused variable.

    #[derive(Debug)]
    #[allow(dead_code)]
    enum RGB {
        Red,
        Green,
        Blue,
    }

    #[allow(dead_code)]
    fn catch_all_with_any_name(color: RGB) -> u8 {
        match color {
            RGB::Red => 0,
            RGB::Green => 1,
            other => {
                // do stuff with variable other optional
                println!("{:?}", other);
                2
            }
        }
    }

    #[allow(dead_code)]
    fn catch_all_with_placeholder(color: RGB) -> u8 {
        match color {
            RGB::Red => 0,
            _ => 1,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Concise Control Flow
////////////////////////////////////////////////////////////////////////////////
mod concise_control_flow {
    //! The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values
    //! that `match` one pattern while ignoring the rest.

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum RGB {
        Red(Brightness),
        Green,
        Blue,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Brightness {
        Dark,
        Light,
    }

    #[allow(dead_code)]
    pub fn red_revolution(color: RGB) -> u8 {
        if let RGB::Red(v) = color {
            println!("{:?}", v);
            0
        } else {
            1
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Custom Discriminant Values
////////////////////////////////////////////////////////////////////////////////
mod custom_discriminant_values {
    //! It is an error when two variants share the same discriminant.
    //! ```text
    //! enum SharedDiscriminantError {
    //!     SharedA = 1,
    //!     SharedB = 1,
    //! ```
    //!
    //! ```text
    //! enum SharedDiscriminantError2 {
    //!     Zero,       // 0
    //!     One,        // 1
    //!     OneToo = 1, // 1 (collision with previous!)
    //! ```
    //!
    //! It is also an error to have an unspecified discriminant where the previous discriminant is
    //! the maximum value for the size of the discriminant.
    //! ```text
    //! #[repr(u8)]
    //! enum OverflowingDiscriminantError {
    //!     Max = 255,
    //!     MaxPlusOne // Would be 256, but that overflows the enum.
    //! }
    //! ```
    //!
    //! ```text
    //! #[repr(u8)]
    //! enum OverflowingDiscriminantError2 {
    //!     MaxMinusOne = 254, // 254
    //!     Max,               // 255
    //!     MaxPlusOne         // Would be 256, but that overflows the enum.
    //! }
    //! ```

    #[repr(u8)]
    #[allow(dead_code)]
    enum Foo {
        Bar,       // 0
        Baz = 254, // 254
        Qux,       // 255
    }

    #[allow(dead_code)]
    pub fn discriminant() {
        assert_eq!(Foo::Bar as u8, 0);
        assert_eq!(Foo::Baz as u8, 254);
        assert_eq!(Foo::Qux as u8, 255);
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_value_in_cents() {
        use crate::patterns_bind_to_values::*;
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alaska)), 25);
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)), 26);
    }

    #[test]
    fn run_plus_one() {
        assert_eq!(crate::matching_with_option::plus_one(Some(5)), Some(6));
        assert_eq!(crate::matching_with_option::plus_one(None), None);
    }

    #[test]
    fn run_red_revolution() {
        use crate::concise_control_flow::*;
        assert_eq!(red_revolution(RGB::Red(Brightness::Dark)), 0);
        assert_eq!(red_revolution(RGB::Red(Brightness::Light)), 0);
        assert_eq!(red_revolution(RGB::Green), 1);
        assert_eq!(red_revolution(RGB::Blue), 1);
    }

    #[test]
    fn run_discriminant() {
        crate::custom_discriminant_values::discriminant();
    }
}
