////////////////////////////////////////////////////////////////////////////////
// Drop Trait
////////////////////////////////////////////////////////////////////////////////
mod drop_trait {
    //! The `Drop` trait only has one method: drop, which is called automatically when an object
    //! goes out of scope. The main use of the `Drop` trait is to free the resources that the
    //! implementor instance owns.
    //!
    //! `Box`, `Vec`, `String`, `File`, and `Process` are some examples of types that implement the
    //! `Drop` trait to free resources. The `Drop` trait can also be manually implemented for any
    //! custom data type.

    struct Droppable {
        pub name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    #[allow(dead_code)]
    pub fn trivial_implementation() {
        let _a = Droppable { name: "a" };

        // block A
        {
            let _b = Droppable { name: "b" };

            // block B
            {
                let _c = Droppable { name: "c" };
                let _d = Droppable { name: "d" };

                println!("Exiting block B");
            }
            println!("Just exited block B");

            println!("Exiting block A");
        }
        println!("Just exited block A");

        // Variable can be manually dropped using the `drop` function
        drop(_a);

        println!("end of the main function");

        // _a won't be dropped again here, because it already has been (manually) dropped
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_trivial_implementation() {
        super::drop_trait::trivial_implementation();
        // ---- testing::run_trivial_implementation stdout ----
        // Exiting block B
        // > Dropping d
        // > Dropping c
        // Just exited block B
        // Exiting block A
        // > Dropping b
        // Just exited block A
        // > Dropping a
        // end of the main function
    }
}
