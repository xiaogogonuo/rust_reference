//! # HashMap
//!
//! The type `HashMap<K, V>` stores a mapping of keys of type K to values of type V using a hashing
//! function, which determines how it places these keys and values into memory.

pub mod create_hash_map {
    use std::collections::HashMap;

    /// Creates an empty HashMap.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until
    /// it is first inserted into.
    pub fn with_new() {
        let _map: HashMap<u8, String> = HashMap::new();
    }

    pub fn with_from() {
        let _map: HashMap<&str, bool> = HashMap::from([("rust", true), ("c++", false)]);
    }

    pub fn with_into() {
        let _map: HashMap<_, _> = [("rust", 2), ("c++", 4)].into();
    }

    /// Creates an empty HashMap with at least the specified capacity.
    ///
    /// The hash map will be able to hold at least capacity elements without reallocating.
    /// This method is allowed to allocate for more elements than capacity. If capacity is 0,
    /// the hash set will not allocate.
    pub fn with_capacity() {
        let _map: HashMap<String, bool> = HashMap::with_capacity(10);
    }
}

pub mod update_hash_map {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [None] is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert() {
        let mut map: HashMap<&str, i8> = HashMap::new();
        assert_eq!(map.insert("rust", 1), None);
        assert_eq!(map.insert("rust", 2), Some(1));
        assert_eq!(map[&"rust"], 2);
    }

    /// If the key does exist in the hash map, the existing value should remain the way it is.
    /// If the key does not exist, insert it and a value for it.
    pub fn entry_to_insert() {
        let mut map: HashMap<&str, u8> = HashMap::new();
        map.entry("rust").or_insert(1);
        assert_eq!(map["rust"], 1);
        map.entry("rust").or_insert(2);
        assert_eq!(map["rust"], 1);
    }

    pub fn entry_for_counting() {
        // ---- testing::run_update_hash_map_entry_for_counting stdout ----
        // hello world about world: 0x10bd8e730
        // hello_0x10bd8e730: 1_0x6000034c0258
        // world_0x10bd8e736: 1_0x6000034c0228
        // about_0x10bd8e73c: 1_0x6000034c0210
        // world_0x10bd8e742: 2_0x6000034c0228
        // [src/lib.rs:67] map = {"world": 2, "hello": 1, "about": 1}
        let text: &str = "hello world about world";
        println!("{}: {:p}", text, text);
        let mut map: HashMap<&str, u32> = HashMap::new();
        for word in text.split_whitespace() {
            let count: &mut u32 = map.entry(word).or_insert(0);
            *count += 1;
            println!("{}_{:p}: {}_{:p}", word, word, *count, count);
        }
        dbg!(map);

        // ---- testing::run_update_hash_map_entry_for_counting stdout ----
        // [src/lib.rs:76] map = {'b': 1, 't': 2, ' ': 1, 'e': 1, 'u': 1, 'r': 1, 's': 2}
        let text: &str = "rust best";
        let mut map: HashMap<char, u32> = HashMap::new();
        for ch in text.chars() {
            map.entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        dbg!(map);
    }

    pub fn entry_and_or_insert_theory() {
        // ---- testing::run_entry_and_or_insert_theory stdout ----
        // {"rust": false}
        //
        // entry_cpp_before: Entry(VacantEntry("cpp"))
        // entry_cpp_afters: Entry(VacantEntry("cpp"))
        // cpp_value: false
        //
        // entry_rust_before: Entry(OccupiedEntry { key: "rust", value: false, .. })
        // rust:false
        // rust:true
        // entry_rust_afters: Entry(OccupiedEntry { key: "rust", value: true, .. })
        // rust_value: true
        //
        // {"cpp": false, "rust": true}
        let mut m: HashMap<&str, bool> = HashMap::new();
        m.insert("rust", false);
        println!("{:?}\n", m);

        let entry_cpp_before: Entry<&str, bool> = m.entry("cpp");
        println!("entry_cpp_before: {:?}", entry_cpp_before);
        // If key not in HashMap, the key's corresponding entry return itself after `add_modify`.
        let entry_cpp_afters: Entry<&str, bool> = entry_cpp_before.and_modify(|c: &mut bool| {
            println!("cpp:{}", *c);
            *c = false;
            println!("cpp:{}", *c);
        });
        println!("entry_cpp_afters: {:?}", entry_cpp_afters);
        // For Vacant Entry, it insert value to the key, and returns the value's mutable reference.
        let cpp_value: &mut bool = entry_cpp_afters.or_insert(false);
        println!("cpp_value: {}\n", cpp_value);

        let entry_rust_before: Entry<&str, bool> = m.entry("rust");
        println!("entry_rust_before: {:?}", entry_rust_before);
        // If key in HashMap, the key's corresponding entry run the closure and return modified entry.
        let entry_rust_afters: Entry<&str, bool> = entry_rust_before.and_modify(|c: &mut bool| {
            println!("rust:{}", *c);
            *c = true;
            println!("rust:{}", *c);
        });
        println!("entry_rust_afters: {:?}", entry_rust_afters);
        // For Occupied Entry, it directly returns the value's mutable reference.
        let rust_value: &mut bool = entry_rust_afters.or_insert(false);
        println!("rust_value: {}\n", rust_value);

        println!("{:?}", m);
    }

    /// Removes a key from map, returning the value at the key if the key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but [Hash] and [Eq] on the borrowed
    /// form must match those for the key type.
    pub fn remove() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "a");
        assert_eq!(map.remove(&1), Some("a"));
        assert_eq!(map.remove(&1), None);
    }
}

pub mod iter_hash_map {
    use std::collections::HashMap;

    pub fn direct_travel() {
        let mut m: HashMap<&str, bool> = HashMap::new();
        m.insert("rust", false);
        m.insert("java", true);
        for (key, val) in m {
            println!("key: {} val: {}", key, val);
        }
    }

    pub fn iter_travel() {
        let mut m: HashMap<&str, bool> = HashMap::new();
        m.insert("rust", false);
        m.insert("java", true);
        for (key, val) in m.iter() {
            println!("key: {} val: {}", key, val);
        }
    }
}

pub mod common_used_method_of_hash_map {
    use std::collections::HashMap;

    /// Returns the number of elements in the map.
    pub fn len() {
        let map: HashMap<i32, &str> = HashMap::new();
        map.len();
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity() {
        let map: HashMap<i32, &str> = HashMap::new();
        map.capacity();
    }

    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "rust");
        assert_eq!(map.contains_key(&1), true);
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "rust");
        assert_eq!(map.get(&1), Some(&"rust"));
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "c++");
        if let Some(v) = map.get_mut(&1) {
            *v = "rust";
            dbg!(&map);
        }
        assert_eq!(map[&1], "rust");
    }
}

pub mod ownership_hash_map {
    //! For types that implement the `Copy` trait, like `i32`, the values are copied into the
    //! hash map. For owned values like `String`, the values will be moved and the hash map will
    //! be the owner of those values.

    use std::collections::HashMap;

    pub fn demonstrated() {
        let field_name: String = String::from("Favorite color");
        let field_value: String = String::from("Blue");

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point.
    }
}

#[cfg(test)]
mod testing {
    #[test]
    fn run_create_hash_map_with_new() {
        crate::create_hash_map::with_new();
    }

    #[test]
    fn run_create_hash_map_with_from() {
        crate::create_hash_map::with_from();
    }

    #[test]
    fn run_create_hash_map_with_into() {
        crate::create_hash_map::with_into();
    }

    #[test]
    fn run_create_hash_map_with_capacity() {
        crate::create_hash_map::with_capacity();
    }

    #[test]
    fn run_update_hash_map_insert() {
        crate::update_hash_map::insert();
    }

    #[test]
    fn run_update_hash_map_entry_to_insert() {
        crate::update_hash_map::entry_to_insert();
    }

    #[test]
    fn run_update_hash_map_entry_for_counting() {
        crate::update_hash_map::entry_for_counting();
    }

    #[test]
    fn run_entry_and_or_insert_theory() {
        crate::update_hash_map::entry_and_or_insert_theory();
    }

    #[test]
    fn run_update_hash_map_remove() {
        crate::update_hash_map::remove();
    }

    #[test]
    fn run_iter_hash_map_direct_travel() {
        crate::iter_hash_map::direct_travel();
    }

    #[test]
    fn run_iter_hash_map_iter_travel() {
        crate::iter_hash_map::iter_travel();
    }

    #[test]
    fn run_common_used_method_of_hash_map_contains_key() {
        crate::common_used_method_of_hash_map::contains_key();
    }

    #[test]
    fn run_common_used_method_of_hash_map_get() {
        crate::common_used_method_of_hash_map::get();
    }

    #[test]
    fn run_common_used_method_of_hash_map_get_mut() {
        crate::common_used_method_of_hash_map::get_mut();
    }
}
