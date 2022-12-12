// compile
// cargo build

// decompile
// objdump -d target/debug/fast_check > fast_check.s

fn main() {
    fast_check();
}

fn fast_check() {
    let s1: String = String::from("rust");
    let s2: String = String::from("china");
    let s3: String = String::from("shanghai");
    let mut ie: ImportantExcerpt = ImportantExcerpt {
        name: &s1,
        home: &s2,
        city: s3,
    };
    dbg!(ie.ignore_struct_lifetime_annotation());
    dbg!(ie._struct_lifetime_annotation());
    dbg!(ie.struct_lifetime_annotation());
    dbg!(ie.announce_and_return_part("ss"));


    let ss = String::from("iu");
    let rr;
    // let _ie;
    let s4 = String::from("java");
    {
        let _ie = ImportantExcerpt {
            name: "xyz",
            home: &ss,
            city: "xxxxxxx".to_string(),
        };
        rr = _ie._announce_and_return_part(&s4);

    }
    println!("xxxxx, {}", rr);
}

pub struct ImportantExcerpt<'a, 'b> {
    name: &'a str,
    home: &'b String,
    city: String,
}

impl ImportantExcerpt<'_, '_> {
    pub fn test(&mut self) -> String {
        self.city = "".to_string();
        "ss".to_string()
    }
}

impl ImportantExcerpt<'_, '_> {
    pub fn excerpt_level(&self) -> u8 {
        1
    }

    pub fn ignore_struct_lifetime_annotation(&self) -> (&str, &String) {
        (self.name, self.home)
    }
}

impl<'a, 'b> ImportantExcerpt<'a, 'b> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.name
    }
}

impl<'a, 'b> ImportantExcerpt<'a, 'b> {
    fn _announce_and_return_part<'c>(&'a self, announcement: &'c str) -> &'c str
        where
            'a: 'c,
    {
        println!("Attention please: {}", announcement);
        self.name
    }
}

impl<'a> ImportantExcerpt<'a, '_> {
    pub fn _struct_lifetime_annotation(&self) -> (&str, &String) {
        (self.name, self.home)
    }
}

impl<'a, 'b> ImportantExcerpt<'a, 'b> {
    pub fn struct_lifetime_annotation(&self) -> (&str, &String) {
        (self.name, self.home)
    }
}

// struct IE<'a> {
//     part: &'a str,
//     next: &'a str,
//     name: String,
//     rank: i32,
//     home: i32,
// }

// impl IE<'_> {
//     fn test(&self) -> &str {
//         // println!("....{}", self.part);
//         self.part
//     }
// }

// impl<'a> IE<'a> {
//     fn aaa(&self, ann: &str, axx: &str) -> &i32 {
//         if self.part.len() > ann.len() {
//             &self.rank
//         } else {
//             &10
//         }
//     }

    // fn demo(&self, x: &i32, y: &i32) -> &i32 {
    //     if x > y {
    //         &self.rank
    //     } else {
    //         &self.home
    //     }
    // }

    // fn announce_and_return_part(&self, announcement: &str) -> &str {
    //     println!("Attention please: {}", announcement);
    //     self.part;
    //     announcement
    // }
// }

// fn demo(x: &i32, y: &i32) -> &i32 {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }

// #[derive(Debug)]
// struct RefRef<'a, 'b: 'a, T> {
//     field: &'a &'b T,
// }
