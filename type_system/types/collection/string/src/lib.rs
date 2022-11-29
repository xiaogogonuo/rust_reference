//! # String Type
//! A `String` is made up of three components: a pointer to some bytes, a length, and a capacity.
//! The pointer points to an internal buffer `String` uses to store its data. The length is the
//! number of bytes currently stored in the buffer, the capacity is the size of the buffer in bytes.

/// A `String` 0~8 bytes store the underline data pointer, 8~16 bytes store capacity, 16~24 bytes
/// store length.
/// ```text
/// -------------- 0x3053bd718
/// 0x7f9f34804080
/// -------------- 0x3053bd720
///      500
/// -------------- 0x3053bd728
///       4
/// -------------- 0x3053bd730
/// ```
pub fn string_memory_layout() {
    let mut s: String = String::with_capacity(500);
    s.push_str("rust");
    let p: *const String = &s;
    println!("string address: {:p}", p); // string address: 0x3053bd718
    println!("underline data address: {:p}", s.as_ptr()); // underline data address: 0x7f9f34804080
    unsafe {
        println!(
            "0~8 bytes store underline pointer: {:#0x?}",
            *(p as *const u64)
        ); // 0~8 bytes store underline pointer: 0x7f9f34804080

        let capacity_address: *const u64 = (p as usize + 8) as *const u64;
        println!(
            "8~16 bytes store capacity: {:p}:{}",
            capacity_address, *capacity_address
        ); // 8~16 bytes store capacity: 0x3053bd720:500

        let length_address: *const u64 = (p as usize + 16) as *const u64;
        println!(
            "16~24 bytes store length: {:p}:{}",
            length_address, *length_address
        ); // 16~24 bytes store length: 0x3053bd728:4
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn size_of_string_in_bytes() {
        assert_eq!(std::mem::size_of::<String>(), 24);
    }

    #[test]
    fn run_string_memory_layout() {
        crate::string_memory_layout();
    }
}
