//! # MIR
//!
//! [Introducing MIR](https://blog.rust-lang.org/2016/04/19/MIR.html)
//! [The MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)
//!
//! ```shell
//! cargo build
//! cargo rustc -- -Z unpretty=mir
//! ```

pub mod mir {
    pub fn let_implicit_scope() {
        let v1: i32 = 1;
        let v2: i32 = v1;
    }
}
