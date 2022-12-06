pub mod concrete_types {
    pub fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest: &i32 = &list[0];
        for i in list {
            if largest < i {
                largest = i
            }
        }
        largest
    }

    pub fn largest_char(list: &[char]) -> &char {
        let mut largest: &char = &list[0];
        for i in list {
            if largest < i {
                largest = i
            }
        }
        largest
    }
}

pub mod generic_types {

    pub mod in_function_definitions {
        //! When we use a parameter in the body of the function, we have to declare the parameter
        //! name in the signature so the compiler knows what that name means. Similarly, when use
        //! a type parameter name in a function signature, we have to declare the type parameter
        //! name before we use it. To define the generic function, place type name declarations
        //! inside angle brackets, `<>`, between the name of the function and the parameter list:
        //! `fn largest<T>(list: &[T]) -> &T {}`

        /// We want to compare values of type T in the body, we can only use types whose values can
        /// be ordered. To enable comparisons, the standard library has the `std::cmp::PartialOrd`
        /// trait that you can implement on types. so we restrict the types valid for T to only
        /// those that implement `PartialOrd`, because the standard library implements `PartialOrd`
        /// on both `i32` and `char`.
        pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut largest: &T = &list[0];
            for i in list {
                if largest < i {
                    largest = i
                }
            }
            largest
        }
    }

    pub mod in_struct_definitions {
        //! The syntax for using generics in struct definitions is similar to that used in function
        //! definitions. First, we declare the name of the type parameter inside angle brackets just
        //! after the name of the struct. Then we use the generic type in the struct definition
        //! where we would otherwise specify concrete data types.
        //!
        //! # Examples
        //!
        //! ```
        //! struct Point<T> {
        //!     x: T,
        //!     y: T,
        //! }
        //!
        //! let i: Point<i32> = Point {x: 1, y: 1};
        //! let f: Point<f32> = Point {x: 1.0, y: 2.0};
        //!
        //! // The fields x and y must be the same type, both have the same generic data type T.
        //! // let wont_work = Point { x: 5, y: 4.0 };
        //! ```
        //!
        //! To define a Point struct where x and y are both generics but could have different types,
        //! we can use multiple generic type parameters.
        //!
        //! # Examples
        //!
        //! ```
        //! struct Point<T, U> {
        //!     x: T,
        //!     y: U,
        //! }
        //!
        //! let i: Point<i32, i32> = Point {x: 1, y: 1};
        //! let f: Point<f32, f32> = Point {x: 1.0, y: 2.0};
        //! let m: Point<i32, f32> = Point {x: 1, y: 2.0};
        //! ```
    }

    pub mod in_enum_definitions {
        //! ```
        //! enum Option<T> {
        //!     Some(T),
        //!     None,
        //! }
        //! ```
        //!
        //! ```
        //! enum Result<T, E> {
        //!     Ok(T),
        //!     Err(E),
        //! }
        //! ```
    }

    pub mod in_method_definitions {
        //! We have to declare `T` just after `impl` so we can use `T` to specify that we’re
        //! implementing methods on the type `Point<T, U>`. By declaring `T` as a generic type
        //! after `impl`, Rust can identify that the type in the angle brackets in `Point` is a
        //! generic type rather than a concrete type. We could have chosen a different name for
        //! this generic parameter than the generic parameter declared in the struct definition,
        //! but using the same name is conventional.
        //!
        //! We can also specify constraints on generic types when defining methods on the type.
        //! For example, implement methods only on `Point<f64, f64>` instances rather than on
        //! `Point<T, U>` instances with any generic type.
        //!
        //! Generic type parameters in a struct definition are not always the same as those you use
        //! in that same struct’s method signatures. For example, uses the generic types `T` and `U`
        //! for the `Point` struct, uses the generic types `X1` and `Y1` for `impl` declaration, and
        //! use `X2` and `Y2` for the `mix_up` method signature to make the example clearer.
        //! The purpose of this is to demonstrate a situation in which some generic parameters are
        //! declared with `impl` and some are declared with the method definition. Here, the generic
        //! parameters `X1` and `Y1` are declared after `impl` because they go with the struct
        //! definition. The generic parameters `X2` and `Y2` are declared after fn `mix_up`, because
        //! they’re only relevant to the method.

        #[derive(Debug)]
        pub struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            pub fn new(x: T, y: U) -> Point<T, U> {
                Point { x, y }
            }

            pub fn from(x: T, y: U) -> Self {
                Self { x, y }
            }

            pub fn moving(self) -> (T, U) {
                (self.x, self.y)
            }

            pub fn borrow(&self) -> (&T, &U) {
                (&self.x, &self.y)
            }
        }

        impl Point<f64, f64> {
            pub fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        impl<X1, Y1> Point<X1, Y1> {
            pub fn mix_up<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {

    #[test]
    fn run_concrete_types_largest_i32() {
        assert_eq!(
            crate::concrete_types::largest_i32(&vec![34, 50, 25, 100, 65]),
            &100
        )
    }

    #[test]
    fn run_concrete_types_largest_char() {
        assert_eq!(
            crate::concrete_types::largest_char(&vec!['y', 'm', 'a', 'q']),
            &'y'
        );
    }

    #[test]
    fn run_generic_types_in_method_definitions() {
        use crate::generic_types::in_method_definitions::Point;
        let p: Point<i32, &str> = Point::new(1, "rust");
        println!("{:?}", p.borrow());
        println!("{:?}", p.moving());

        let pc: Point<f64, f64> = Point::from(3.0, 4.0);
        println!("{:?}", pc.distance_from_origin());

        let p1: Point<i32, f64> = Point::new(5, 10.4);
        let p2: Point<&str, char> = Point::new("rust", 'c');
        let p3: Point<i32, char> = p1.mix_up(p2);
        println!("p3 = {:?}", p3);
    }
}
