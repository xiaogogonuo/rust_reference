////////////////////////////////////////////////////////////////////////////////
// Defining And Instantiating Structs
////////////////////////////////////////////////////////////////////////////////
mod defining_and_instantiating_structs {
    #[allow(dead_code)]
    struct Laptop {
        name: String,
        size: usize,
    }

    #[allow(dead_code)]
    fn instantiating_structs() {
        Laptop {
            name: "mac".to_string(),
            size: 13,
        };
    }

    #[allow(dead_code)]
    fn read_structs_filed() {
        Laptop {
            name: "mac".to_string(),
            size: 13,
        }
        .name;
    }

    #[allow(dead_code)]
    fn write_structs_filed() {
        Laptop {
            name: "mac".to_string(),
            size: 13,
        }
        .size = 16;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Field Init Shorthand
////////////////////////////////////////////////////////////////////////////////
mod filed_init_shorthand {
    //! Because the parameter names and the struct field names are exactly the same, we can use the
    //! field init shorthand syntax.

    #[allow(dead_code)]
    struct Laptop {
        name: String,
        size: usize,
    }

    #[allow(dead_code)]
    fn short_hand_for_parameter(name: String) -> Laptop {
        Laptop { name, size: 13 } // same as: `Laptop { name: name, size: 13 }`
    }
}

////////////////////////////////////////////////////////////////////////////////
// Struct Update Syntax
////////////////////////////////////////////////////////////////////////////////
mod struct_update_syntax {
    //! It’s often useful to create a new instance of a struct that includes most of the values from
    //! another instance, but changes some. You can do this using struct update syntax.
    //!
    //! The syntax `..` specifies that the remaining fields not explicitly set should have the same
    //! value as the fields in the given instance.

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Laptop {
        name: String,
        size: usize,
    }

    #[allow(dead_code)]
    fn create_new_instance_without_struct_update_syntax() {
        let laptop = Laptop {
            name: "mac".to_string(),
            size: 13,
        };

        let _new_laptop = Laptop {
            name: "dell".to_string(),
            size: laptop.size,
        };
        println!("{:#?}", laptop);

        let _new_laptop = Laptop {
            name: laptop.name, // value partially moved here
            size: 16,
        };
        // value borrowed here after partial move, panic if you use like below:
        // println!("{:#?}", laptop);
    }

    #[allow(dead_code)]
    fn create_new_instance_with_struct_update_syntax() {
        let laptop = Laptop {
            name: "mac".to_string(),
            size: 13,
        };

        let _new_laptop = Laptop { size: 16, ..laptop };
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tuple Structs
////////////////////////////////////////////////////////////////////////////////
mod tuple_structs {
    //! Using Tuple Structs without Named Fields to Create Different Types.
    //!
    //! Tuple structs are useful when you want to give the whole tuple a name and make the tuple
    //! a different type from other tuples.

    #[allow(dead_code)]
    struct Color(i32, i32, i32);

    #[allow(dead_code)]
    struct Point(i32, i32, i32);

    #[allow(dead_code)]
    fn instantiating_tuple_structs() {
        Color(255, 255, 255);
        Point(0, 0, 0);
    }

    #[allow(dead_code)]
    fn read_tuple_structs_field() {
        Color(255, 255, 255).0;
        Color(255, 255, 255).1;
        Color(255, 255, 255).2;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Unit Like Structs
////////////////////////////////////////////////////////////////////////////////
mod unit_like_structs {
    //! Unit-like structs can be useful when you need to implement a trait on some type but don’t
    //! have any data that you want to store in the type itself.

    #[derive(Debug)]
    #[allow(dead_code)]
    struct AlwaysEqual;

    #[allow(dead_code)]
    fn instantiating_unit_like_structs() {
        let subject = AlwaysEqual;
        println!("{:#?}", subject);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Defining Methods
////////////////////////////////////////////////////////////////////////////////
mod defining_methods {
    //! Methods are defined within the context of a struct(or an enum or a trait object), and their
    //! first parameter is always `self`, which represents the instance of the struct.
    //!
    //! Methods must have a parameter named `self` of type `Self` for their first parameter.
    //!
    //! The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is
    //! an alias for the type that the `impl` block is for.
    //!
    //! Use `&` in front of the `self` to indicate this method borrows the `Self` instance.
    //! Use `&mut` in front of the `self` to change the `Self` instance.

    #[allow(dead_code)]
    struct Laptop {
        name: String,
        size: usize,
    }

    impl Laptop {
        #[allow(dead_code)]
        fn size(&self) -> usize {
            self.size
        }

        #[allow(dead_code)]
        fn alter_size(&mut self, new_size: usize) {
            self.size = new_size;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Associated Functions
////////////////////////////////////////////////////////////////////////////////
mod associated_functions {
    //! All functions defined within an `impl` block are called associated functions.
    //!
    //! Associated functions that are not methods are often used for constructors that will return
    //! a new instance of the struct, these are often called new.
    //!
    //! The `Self` keywords in the return type and in the body of the function are aliases for the
    //! type that appears after the `impl` keyword, which in this case is Player.
    //!
    //! To call this associated function, we use the `::` syntax with the struct name; for example:
    //! `let player = Player::new("rust".to_string(), "1".to_string());`

    #[allow(dead_code)]
    struct Player {
        name: String,
        rank: String,
    }

    impl Player {
        #[allow(dead_code)]
        fn new(name: String, rank: String) -> Self {
            Self { name, rank }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Multiple Impl Blocks
////////////////////////////////////////////////////////////////////////////////
mod multiple_impl_blocks {
    //! There’s no reason to separate methods into multiple `impl` blocks, but this is valid syntax.

    #[allow(dead_code)]
    struct Player {
        name: String,
        rank: String,
    }

    impl Player {
        #[allow(dead_code)]
        fn new(name: String, rank: String) -> Self {
            Self { name, rank }
        }
    }

    impl Player {
        #[allow(dead_code)]
        fn name_len(&self) -> usize {
            self.name.len()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Make Struct Field Public
////////////////////////////////////////////////////////////////////////////////
mod make_struct_field_public {
    //! Like variable in Rust, field in struct is default private, we can us `pub` to let it public.

    mod inner {
        #[allow(dead_code)]
        pub struct Player {
            pub name: String,
            rank: usize,
        }

        impl Player {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_string(),
                    rank: 1,
                }
            }

            pub fn rank(&self) -> usize {
                self.rank
            }
        }
    }

    #[allow(dead_code)]
    fn read_public_filed() {
        inner::Player::new("rust").name;
    }

    #[allow(dead_code)]
    fn read_private_filed() {
        inner::Player::new("rust").rank();
    }
}

pub mod memory_layout {

    #[allow(dead_code)]
    struct OneBytes {
        f: i8,
    }

    pub fn size_of_struct_in_one_bytes() {
        assert_eq!(std::mem::size_of::<OneBytes>(), 1);
    }

    #[allow(dead_code)]
    struct TwoBytes {
        f: i16,
    }

    pub fn size_of_struct_in_two_bytes() {
        assert_eq!(std::mem::size_of::<TwoBytes>(), 2);
    }

    #[allow(dead_code)]
    struct FourBytes {
        f: char,
    }

    pub fn size_of_struct_in_four_bytes() {
        assert_eq!(std::mem::size_of::<FourBytes>(), 4);
    }

    #[allow(dead_code)]
    struct EightBytes {
        f: i64,
    }

    pub fn size_of_struct_in_eight_bytes() {
        assert_eq!(std::mem::size_of::<EightBytes>(), 8);
    }

    #[allow(dead_code)]
    struct TwentyFourBytes {
        f: String,
    }

    pub fn size_of_struct_in_twenty_four_bytes() {
        assert_eq!(std::mem::size_of::<TwentyFourBytes>(), 24);
    }

    #[allow(dead_code)]
    struct MixedBytes {
        f1: u8,
        f2: i16,
        f3: char,
        f4: String,
        f5: String,
        f6: u8,
        f7: u16,
    }

    pub fn size_of_struct_in_mixed_bytes() {
        // ---- testing::size_of_struct_in_bytes stdout ----
        // f1: 0x70000DF92670
        // f2: 0x70000DF9266C
        // f3: 0x70000DF92668
        // f4: 0x70000DF92638
        // f5: 0x70000DF92650
        // f6: 0x70000DF92671
        // f7: 0x70000DF9266E

        /*

        0x70000DF92638 ～ 0x70000DF9263F     |<-- 8 BYTES -->|       ---
                                                                      |
        0x70000DF92640 ～ 0x70000DF92647     |<-- 8 BYTES -->|        f4
                                                                      |
        0x70000DF92648 ～ 0x70000DF9264F     |<-- 8 BYTES -->|       ---

        0x70000DF92650 ～ 0x70000DF92657     |<-- 8 BYTES -->|       ---
                                                                      |
        0x70000DF92658 ～ 0x70000DF9265F     |<-- 8 BYTES -->|        f5
                                                                      |
        0x70000DF92660 ～ 0x70000DF92667     |<-- 8 BYTES -->|       ---

                                                    f3               f2               f7
        0x70000DF92668 ～ 0x70000DF9266F     |<-- 4 BYTES -->||<-- 2 BYTES -->||<-- 2 BYTES -->|

                                                    f1               f6               padding
        0x70000DF92670 ～ 0x70000DF92677     |<-- 1 BYTES -->||<-- 1 BYTES -->||<-- 6 BYTES -->|
         */
        let m = MixedBytes {
            f1: 1,
            f2: 1,
            f3: 'x',
            f4: "rust".to_string(),
            f5: "c++".to_string(),
            f6: 1,
            f7: 1,
        };
        println!("f1: 0x{:X}", &m.f1 as *const u8 as usize);
        println!("f2: 0x{:X}", &m.f2 as *const i16 as usize);
        println!("f3: 0x{:X}", &m.f3 as *const char as usize);
        println!("f4: 0x{:X}", &m.f4 as *const String as usize);
        println!("f5: 0x{:X}", &m.f5 as *const String as usize);
        println!("f6: 0x{:X}", &m.f6 as *const u8 as usize);
        println!("f7: 0x{:X}", &m.f7 as *const u16 as usize);

        assert_eq!(std::mem::size_of::<MixedBytes>(), 64);
    }
}

#[cfg(test)]
pub mod testing {

    #[test]
    fn size_of_struct_in_bytes() {
        crate::memory_layout::size_of_struct_in_one_bytes();
        crate::memory_layout::size_of_struct_in_two_bytes();
        crate::memory_layout::size_of_struct_in_four_bytes();
        crate::memory_layout::size_of_struct_in_eight_bytes();
        crate::memory_layout::size_of_struct_in_twenty_four_bytes();
        crate::memory_layout::size_of_struct_in_mixed_bytes();
    }
}
