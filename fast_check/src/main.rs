// compile
// cargo build

// decompile
// objdump -d target/debug/fast_check > fast_check.s

#[derive(Debug)]
struct RustContext<'a, 'b> {
    name: &'a str,
    vars: Vec<&'b str>,
}

struct Context<'a> {
    name: &'a str,
}

impl<'a> Context<'a> {
    pub fn danger_mode(&self) -> &str {
        self.name
    }

    pub fn safety_mode(&self) -> &'a str {
        self.name
    }
}

fn best_case() {
    let s: String = String::from("rust");
    let danger_ref: &str;
    let safety_ref: &str;

    {
        let a = Context { name: &s };
        danger_ref = a.danger_mode();
        safety_ref = a.safety_mode();
        println!("danger: {}", danger_ref);
    }
    println!("safety: {}", safety_ref);




    // let mut rc: RustContext = RustContext {
    //     name: "",
    //     vars: vec![],
    // };

    // rc.vars.push("rust");
    // {
    //     let s: String = String::from("go");
    //     rc.vars.push(&s);
    //     println!("{:?}", rc);
    // }
    //
    // {
    //     rc.name = "rs";
    //     println!("{:?}", rc.name);
    // }
}

fn main() {
    best_case();
    // fast_check();

    // let mario = 1;
    // let ref1 = &mario;
    // let p: &i32;
    // {
    //     let luigi = 2;
    //     let ref2 = &luigi;
    //     {
    //         // let peach = do_sth(&ref2, &ref1);
    //         // println!("{}", peach);
    //
    //         p = do_sth(&ref2, &ref1);
    //         println!("{}", p);
    //     }
    // }
    // let c: Context;
    // {
    //     let name = "rust".to_string();
    //     {
    //         let s = "c++".to_string();
    //         let vars = vec![s.as_str()];
    //         c = Context {
    //             name: name.as_str(),
    //             vars,
    //         };
    //         println!("{:?}", c);
    //     }
    // }
    //
    // let s = String::from("hello");
    // let s_ref;
    //
    // {
    //     let a = A { name: &s };
    //     s_ref = a.get();
    // }
    // println!("{:?}", s_ref);
}

struct A<'a> {
    name: &'a str,
}

impl<'a> A<'a> {
    fn get(&self) -> &str {
        self.name
    }
}

fn do_sth<'a, 'b, 'c>(arg1: &'a i32, arg2: &'b i32) -> &'c i32
where
    'a: 'b,
    'b: 'c,
{
    arg2
}

// #[derive(Debug)]
// struct Context<'a, 'b> {
//     name: &'a str,
//     vars: Vec<&'b str>,
// }

// #[derive(Debug)]
// struct Context<'a> {
//     name: &'a str,
//     vars: Vec<&'a str>,
// }

// struct Context<'a>(&'a str);
//
// struct Parser<'a> {
//     context: &'a Context<'a>,
// }
//
// impl<'a> Parser<'a> {
//     fn parse(&self) -> Result<(), &'a str> {
//         Err(&self.context.0[1..])
//     }
// }
//
// fn parse_context<'a>(context: &'a Context<'a>) -> Result<(), &'a str> {
//     Parser { context }.parse()
// }

// fn fast_check() {
//     let s1: String = String::from("rust");
//     let s2: String = String::from("china");
//     let ie: ImportantExcerpt = ImportantExcerpt {
//         name: &s1,
//         home: &s2,
//     };
//     dbg!(ie.ignore_struct_lifetime_annotation());
//     dbg!(ie._struct_lifetime_annotation());
//     dbg!(ie.struct_lifetime_annotation());
// }
//
// pub struct ImportantExcerpt<'a, 'b> {
//     name: &'a str,
//     home: &'b String,
// }
//
// impl ImportantExcerpt<'_, '_> {
//     pub fn ignore_struct_lifetime_annotation(&self) -> (&str, &String) {
//         (self.name, self.home)
//     }
// }
//
// impl<'a> ImportantExcerpt<'a, '_> {
//     pub fn _struct_lifetime_annotation(&self) -> (&str, &String) {
//         (self.name, self.home)
//     }
// }
//
// impl<'a, 'b> ImportantExcerpt<'a, 'b> {
//     pub fn struct_lifetime_annotation(&self) -> (&str, &String) {
//         (self.name, self.home)
//     }
// }

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
