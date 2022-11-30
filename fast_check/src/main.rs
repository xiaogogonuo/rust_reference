use std::collections::HashMap;

fn main() {
   let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "rust");
    map.insert(2, "c++");
    for (k, v) in &map {
        if let Some(v) = map.get_mut(k) {
            (*v).repeat(1);
        }
    }
    dbg!(map);
}



// cargo build
// objdump -d target/debug/fast_check > fast_check.s
