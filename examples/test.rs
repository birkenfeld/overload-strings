#![feature(plugin, type_ascription)]
#![plugin(overload_strings)]
#![allow(dead_code)]

pub mod fns {
    use std::borrow::Cow;

    pub fn takes_str(s: &str) {
        print!("{}", s);
    }

    pub fn takes_string(s: String) {
        print!("{}", s);
    }

    pub fn takes_cow(s: Cow<str>) {
        print!("{}", s);
    }

    pub fn takes_into_string<I: Into<String>>(s: I) {
        print!("{}", s.into());
    }
}

#[overload_strings]
mod foo {
    use fns::*;

    static DONT_TOUCH: &'static str = "foo";
    const DONT_TOUCH_EITHER: &'static str = "foo";

    #[overload_strings] // the duplicate is ignored
    pub fn print_it() {
        takes_str("Hello");
        takes_string(", ");
        takes_cow("World");
        // this one can't be inferred, so use ascription
        takes_into_string("!\n": &str);
    }
}

#[overload_strings]
mod bar {
    mod sub {
        use fns::*;

        fn bar() {
            // this works because we don't recurse overloading into submodules
            takes_into_string("...");
        }
    }
}

fn main() {
    foo::print_it();
}
