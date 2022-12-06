mod drop;

pub mod define_trait {
    //! A trait can have multiple methods in its body: the method signatures are listed one per line
    //! and each line ends in a semicolon.

    pub trait Summary {
        fn summarize(&self) -> String;
    }
}

pub mod implement_trait_on_types {
    //! Implementing a trait on a type is similar to implementing regular methods. The difference is
    //! that after impl, we put the trait name we want to implement, then use the for keyword, and
    //! then specify the name of the type we want to implement the trait for. Within the impl block,
    //! we put the method signatures that the trait definition has defined. Instead of adding a
    //! semicolon after each signature, we use curly brackets and fill in the method body with the
    //! specific behavior that we want the methods of the trait to have for the particular type.

    use super::define_trait::Summary;

    pub struct Facebook {
        pub headline: String,
        pub author: String,
    }

    impl Summary for Facebook {
        // can't add pub keyword before trait method for some type
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }

    pub struct Tweet<T> {
        pub reply: T,
        pub retweet: T,
    }

    // The generic parameters must be declared whatever it used or not
    impl<T: std::fmt::Display> Summary for Tweet<T> {
        fn summarize(&self) -> String {
            format!("{}: {}", self.reply, self.retweet)
        }

        // Can't define method not belongs to trait Summary, like:
        // fn not_member_of_trait_summary(&self) {}
    }
}

pub mod orphan_rule {
    //! ‼️ important ‼️
    //!
    //! Other crates that depend on the your crate can also bring the Summary trait into scope to
    //! implement Summary on their own types. One restriction to note is that we can implement a
    //! trait on a type only if at least one of the trait or the type is local to our crate.
    //! For example, we can implement standard library traits like Display on a custom type like
    //! Tweet as part of our crate functionality, because the type Tweet is local to our crate.
    //! We can also implement Summary on Vec<T> in our crate, because the trait Summary is local to
    //! our crate.
    //!
    //! But we can’t implement external traits on external types. For example, we can’t implement
    //! the Display trait on Vec<T> within our crate, because Display and Vec<T> are both defined
    //! in the standard library and are not local to our crate. This restriction is part of a
    //! property called coherence, and more specifically the orphan rule, so named because the
    //! parent type is not present. This rule ensures that other people’s code can’t break your code
    //! and vice versa. Without the rule, two crates could implement the same trait for the same
    //! type, and Rust would not know which implementation to use.

    pub mod implement_external_trait_on_local_type {
        pub struct Position {
            longitude: f32,
            latitude: f32,
        }

        impl Position {
            pub fn new(longitude: f32, latitude: f32) -> Self {
                Self {
                    longitude,
                    latitude,
                }
            }
        }

        impl std::fmt::Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.longitude, self.latitude)
            }
        }
    }

    pub mod implement_local_trait_on_external_type {
        use crate::define_trait::Summary;
        use std::collections::HashMap;

        impl<K, V> Summary for HashMap<K, V>
        where
            K: std::fmt::Display,
            V: std::fmt::Display,
        {
            fn summarize(&self) -> String {
                let mut s: String = String::new();
                for (k, v) in self {
                    s.push_str(&format!("{}{}", k, v))
                }
                s
            }
        }
    }
}

pub mod default_implementation {
    //! Sometimes it’s useful to have default behavior for some or all of the methods in a trait
    //! instead of requiring implementations for all methods on every type.
    //!
    //! To use a default implementation to summarize instances of `Facebook`, we specify an empty
    //! impl block with `impl Summary for Facebook {}`.
    //!
    //! We can still override default implementation to special behavior.

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct Facebook {}

    impl Summary for Facebook {}

    pub struct Tweet {}

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            String::from("override default behavior")
        }
    }
}

pub mod trait_as_parameter {
    pub trait Laptop {
        fn name(&self) -> &str;
    }

    pub struct Apple {
        name: String,
    }

    impl Laptop for Apple {
        fn name(&self) -> &str {
            self.name.as_str()
        }
    }

    pub fn notify(laptop: &impl Laptop) {
        laptop.name();
    }
}

pub mod trait_bound_syntax {
    //! The impl Trait syntax works for straightforward cases but is actually syntax sugar for a
    //! longer form known as a trait bound.

    pub trait Reader {
        fn read(&self) -> &str;
    }

    /// it equivalents to `pub fn request(r: &impl Reader) { r.read(); }`
    pub fn request<T: Reader>(r: &T) {
        r.read();
    }

    pub fn simple_way(r1: &impl Reader, r2: &impl Reader) {
        r1.read();
        r2.read();
    }

    pub fn better_way<T: Reader>(r1: &T, r2: &T) {
        r1.read();
        r2.read();
    }
}

pub mod multiple_trait_bounds_with_plus_syntax {
    use std::fmt::{Debug, Display};

    pub trait Reader {
        fn read(&self) -> &str;
    }

    pub fn simple_way(r: &(impl Reader + Display)) {
        r.read();
    }

    pub fn better_way<T: Reader + Display>(r: &T) {
        r.read();
    }

    pub fn where_clause<T>(r: &T)
    where
        T: Reader + Display,
    {
        r.read();
    }

    pub fn _some<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {
        // do stuff with _t and _u
    }

    pub fn some_<T, U>(_t: &T, _u: &U)
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        // do stuff with _t and _u
    }
}

pub mod return_type_implement_trait {
    pub trait Reader {
        fn read(&self) -> &str;
    }

    pub struct Office {}

    impl Reader for Office {
        fn read(&self) -> &str {
            "rust"
        }
    }

    pub fn return_reader() -> impl Reader {
        Office {}
    }
}

pub mod use_trait_bound_to_conditionally_implement_methods {
    //! By using a trait bound with an impl block that uses generic type parameters, we can
    //! implement methods conditionally for types that implement the specified traits.

    use std::fmt::Display;

    pub struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        pub fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use std::collections::HashMap;

    // trait must be added to the scope where some type calls the method of the trait
    use crate::define_trait::Summary;

    use crate::implement_trait_on_types::{Facebook, Tweet};
    use crate::orphan_rule::implement_external_trait_on_local_type::Position;

    #[test]
    fn run_implement_trait_on_types() {
        let facebook: Facebook = Facebook {
            headline: "".to_string(),
            author: "".to_string(),
        };
        facebook.summarize();

        let tweet: Tweet<char> = Tweet {
            reply: 'c',
            retweet: '+',
        };
        tweet.summarize();
    }

    #[test]
    fn run_orphan_rule_implement_external_trait_on_local_type() {
        let position: Position = Position::new(1.0, 2.0);
        println!("{}", position);
    }

    #[test]
    fn run_orphan_rule_implement_local_trait_on_external_type() {
        let map: HashMap<&str, char> = HashMap::from([("rust", 'A'), ("c++", 'B')]);
        println!("{}", map.summarize());
    }

    #[test]
    fn run_default_implementation() {
        use crate::default_implementation::{Facebook, Summary, Tweet};
        println!("{}", Facebook {}.summarize());
        println!("{}", Tweet {}.summarize());
    }
}
