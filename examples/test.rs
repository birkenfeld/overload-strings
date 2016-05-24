#![feature(plugin, type_ascription)]
#![plugin(overload_strings)]
#![allow(dead_code)]

use std::borrow::Cow;

static DONT_WORK: &'static str = "foo";

fn takes_str(s: &str) {
    print!("{}", s);
}

fn takes_string(s: String) {
    print!("{}", s);
}

fn takes_cow(s: Cow<str>) {
    print!("{}", s);
}

fn takes_into_string<I: Into<String>>(s: I) {
    print!("{}", s.into());
}

#[overload_strings]
fn main() {
    takes_str("Hello");
    takes_string(", ");
    takes_cow("World");
    // this one can't be inferred, so use ascription
    takes_into_string("\n": &str);
}
