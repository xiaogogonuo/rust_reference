//! # Vector
//!
//! Vectors allow you to store more than one value in a single data structure that puts all the
//! values next to each other in memory. Vectors can only store values of the same type.

/// A `Vector` 0~8 bytes store the underline data pointer, 8~16 bytes store capacity, 16~24 bytes
/// store length.
/// ```text
/// -------------- 0x30c863718
/// 0x600000cf0000
/// -------------- 0x30c863720
///      500
/// -------------- 0x30c863728
///       1
/// -------------- 0x30c863730
/// ```
pub fn vector_memory_layout() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    let p: *const Vec<i32> = &v;

    // ---- testing::run_vector_memory_layout stdout ----
    // vector address: 0x30c863718
    // underline data address: 0x600000cf0000
    // 0~8 bytes store underline pointer: 0x600000cf0000
    // 8~16 bytes store capacity: 0x30c863720:10
    // 16~24 bytes store length: 0x30c863728:1
    println!("vector address: {:p}", p);
    println!("underline data address: {:p}", v.as_ptr());
    unsafe {
        println!(
            "0~8 bytes store underline pointer: {:#0x?}",
            *(p as *const u64)
        );

        let capacity_address: *const u64 = (p as usize + 8) as *const u64;
        println!(
            "8~16 bytes store capacity: {:p}:{}",
            capacity_address, *capacity_address
        );

        let length_address: *const u64 = (p as usize + 16) as *const u64;
        println!(
            "16~24 bytes store length: {:p}:{}",
            length_address, *length_address
        );
    }
}

pub mod create_vector {
    pub fn with_new() {
        // type annotation is needed here, because we are not inserting any values into this vector
        let _immutable_vector: Vec<&str> = Vec::new();

        let mut mutable_vector: Vec<&str> = Vec::new();
        mutable_vector.push("rust");
    }

    pub fn with_capacity() {
        // type annotation is needed here, because we are not inserting any values into this vector
        let _immutable_vector: Vec<&str> = Vec::with_capacity(0);

        let mut mutable_vector: Vec<&str> = Vec::with_capacity(10);
        mutable_vector.push("rust");
    }

    pub fn with_marco() {
        // type annotation is needed here, because we are not inserting any values into this vector
        let _immutable_vector: Vec<&str> = vec![];

        let mut mutable_vector: Vec<&str> = vec!["c++"];
        mutable_vector.push("rust");
    }
}

pub mod update_vector {
    pub fn push() {
        let mut v: Vec<i8> = vec![];
        v.push(1);
    }

    pub fn pop() {
        let mut v: Vec<i8> = vec![1, 3, 5, 7, 9];
        while let Some(x) = v.pop() {
            println!("{} pop out.", x);
        }
        if let None = v.pop() {
            println!("vector is empty.");
        }
    }
}

pub mod read_vector {
    //! Two ways to reference a value stored in a vector: via indexing or using the get method.

    pub fn with_index() {
        // get value by index
        let immutable_vector: Vec<i8> = vec![1, 2, 3];
        let first_value: i8 = immutable_vector[0];
        assert_eq!(first_value, 1);

        // get immutable reference by index
        let immutable_vector: Vec<i8> = vec![1, 2, 3];
        let first_value: &i8 = &immutable_vector[0];
        assert_eq!(first_value, &1);

        // get mutable reference by index
        let mut mutable_vector: Vec<i8> = vec![1, 2, 3];
        let first_value: &mut i8 = &mut mutable_vector[0];
        assert_eq!(first_value, &1);

        // if the index exceeds the vector length, it will cause the program to panic
        // let v: Vec<i8> = vec![1];
        // v[10];
    }

    /// `get` method returns a reference to an element or sub slice depending on the type of index.
    pub fn with_get() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];

        let first_value: Option<&i32> = v.get(0);
        assert_eq!(first_value, Some(&1));

        let range_value: Option<&[i32]> = v.get(0..3);
        assert_eq!(range_value, Some(&[1, 2, 3][..]));

        let not_exist_value: Option<&i32> = v.get(100);
        assert_eq!(not_exist_value, None);

        let not_exist_range: Option<&[i32]> = v.get(0..10);
        assert_eq!(not_exist_range, None);

        let v: Vec<&str> = vec!["c", "c++", "rust"];
        assert_eq!(v.get(0), Some(&"c"));
        let rust: Option<&&str> = v.get(2);
        match rust {
            Some(v) => println!("{}", v),
            None => println!("not rust"),
        }
    }
}

pub mod iter_vector {
    pub fn read() {
        let v: Vec<i8> = vec![1, 2, 3, 4, 5];

        // ---- testing::run_iter_vector_read stdout ----
        // 0x6000001c8000 : 1
        // 0x6000001c8001 : 2
        // 0x6000001c8002 : 3
        // 0x6000001c8003 : 4
        // 0x6000001c8004 : 5
        for x in &v {
            println!("{:p} : {}", x, x);
        }

        // ---- testing::run_iter_vector_read stdout ----
        // 0x30e0b784f : 1
        // 0x30e0b784f : 2
        // 0x30e0b784f : 3
        // 0x30e0b784f : 4
        // 0x30e0b784f : 5
        for x in v {
            println!("{:p} : {}", &x, x);
        }
    }

    pub fn update() {
        let mut v: Vec<i32> = vec![1, 2, 3];

        // ---- testing::run_iter_vector_update stdout ----
        // 0x600001f94000 : 1
        // 0x600001f94004 : 2
        // 0x600001f94008 : 3
        for x in &mut v {
            println!("{:p} : {}", x, x);
            // To change the value that the mutable reference refers to, we have to use the *
            // dereference operator to get to the value in i before we can use the += operator.
            *x += 1;
        }
        assert_eq!(v, vec![2, 3, 4]);
    }
}

pub mod drop_vector {
    //! Like any other struct, a vector is freed when it goes out of scope.
    //!
    //! {
    //!     let v = vec![1, 2, 3, 4];
    //!     // do stuff with v
    //! } // v goes out of scope and is freed here
    //!
    //! When the vector gets dropped, all of its contents are also dropped, meaning the integers it
    //! holds will be cleaned up. The borrow checker ensures that any references to contents of a
    //! vector are only used while the vector itself is valid.

    pub fn explicit_call() {
        let v: Vec<i8> = vec![1, 2, 3];
        std::mem::drop(v);

        // can't use v afterward
        // println!("{:?}", v);
    }
}

pub mod use_enum_to_store_multiple_types {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn spread_sheet_cell() {
        let _row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}

pub mod vector_trap {
    //! We hold an immutable reference to the first element in a vector and try to add an element to
    //! the end. This program won’t work if we also try to refer to that element later.
    //!
    //! ```panic
    //! let mut v = vec![1, 2, 3, 4, 5];
    //! let first = &v[0];
    //! v.push(6);
    //! println!("The first element is: {}", first);
    //! ```
    //!
    //! Why should a reference to the first element care about changes at the end of the vector?
    //! This error is due to the way vectors work: because vectors put the values next to each other
    //! in memory, adding a new element onto the end of the vector might require allocating new
    //! memory and copying the old elements to the new space, if there isn’t enough room to put all
    //! the elements next to each other where the vector is currently stored. In that case, the
    //! reference to the first element would be pointing to deallocated memory. The borrowing rules
    //! prevent programs from ending up in that situation.

    pub fn iter_vector() {
        let v: Vec<i32> = vec![1, 2, 3];
        for _ in v {} // `v` moved due to this implicit call to `.into_iter()`

        // can't use v afterward
        // println!("{:?}", v);
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_vector_in_bytes() {
        assert_eq!(std::mem::size_of::<Vec<i32>>(), 24);
        assert_eq!(std::mem::size_of::<Vec<i16>>(), 24);
    }

    #[test]
    fn run_vector_memory_layout() {
        crate::vector_memory_layout()
    }

    #[test]
    fn run_create_vector_with_new() {
        crate::create_vector::with_new();
    }

    #[test]
    fn run_create_vector_with_capacity() {
        crate::create_vector::with_capacity();
    }

    #[test]
    fn run_create_vector_with_marco() {
        crate::create_vector::with_marco();
    }

    #[test]
    fn run_update_vector_push() {
        crate::update_vector::push();
    }

    #[test]
    fn run_update_vector_pop() {
        crate::update_vector::pop();
    }

    #[test]
    fn run_read_vector_with_index() {
        crate::read_vector::with_index();
    }

    #[test]
    fn run_read_vector_with_get() {
        crate::read_vector::with_get();
    }

    #[test]
    fn run_iter_vector_read() {
        crate::iter_vector::read();
    }

    #[test]
    fn run_iter_vector_update() {
        crate::iter_vector::update();
    }
}
