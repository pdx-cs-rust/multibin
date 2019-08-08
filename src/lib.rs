//! Put code common to all binaries in this library
//! crate. Put modules of this library crate in this
//! directory.

pub fn say_hello(who: &str) {
    println!("hello {}", who);
}
