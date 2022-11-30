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
    pub fn entry() {
        let mut map: HashMap<&str, u8> = HashMap::new();
        map.entry("rust").or_insert(1);
        assert_eq!(map["rust"], 1);
        map.entry("rust").or_insert(2);
        assert_eq!(map["rust"], 1);

        // ---- testing::run_update_hash_map_entry stdout ----
        // hello world about world: 0x102d6f180
        // hello_0x102d6f180: 0x600000d2c1d8
        // world_0x102d6f186: 0x600000d2c1a8
        // about_0x102d6f18c: 0x600000d2c190
        // world_0x102d6f192: 0x600000d2c1a8
        // [src/lib.rs:75] map = {
        //     "hello": 1,
        //     "world": 2,
        //     "about": 1,
        // }
        {
            let text: &str = "hello world about world";
            println!("{}: {:p}", text, text);

            let mut map: HashMap<&str, u32> = HashMap::new();

            for word in text.split_whitespace() {
                // The `or_insert` method returns a mutable reference (&mut V) to the value for the
                // specified key.
                let count = map.entry(word).or_insert(0);
                println!("{}_{:p}: {:p}", word, word, count);
                *count += 1;
            }

            dbg!(map);
        }
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

#[cfg(test)]
mod testing {
    #[test]
    fn run_create_hash_map_with_new() {
        crate::create_hash_map::with_new();
    }

    #[test]
    fn run_update_hash_map_insert() {
        crate::update_hash_map::insert();
    }

    #[test]
    fn run_update_hash_map_entry() {
        crate::update_hash_map::entry();
    }

    #[test]
    fn run_update_hash_map_remove() {
        crate::update_hash_map::remove();
    }

    #[test]
    fn run_update_hash_map_contains_key() {
        crate::update_hash_map::contains_key();
    }

    #[test]
    fn run_update_hash_map_get() {
        crate::update_hash_map::get();
    }

    #[test]
    fn run_update_hash_map_get_mut() {
        crate::update_hash_map::get_mut();
    }
}
