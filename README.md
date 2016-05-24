# overload-strings

This is a quick and silly syntax extension, mostly to familiarize myself with
compiler plugins using custom attributes.  It does the equivalent of Haskell's
`OverloadedStrings`, i.e. it inserts an `.into()` call onto every string
literal.  No extra trait is necessary.

## Usage

As a compiler plugin, requires nightly Rust.  Add
```rust
#![feature(plugin)]
#![plugin(overload_strings)]
```
then apply the `#[overload_strings]` attribute on the item(s) you want to
overload string literals in (module, fn, impl, ...).

The annotation does *not* automatically recurse into submodules, to keep
surprises due to nonlocal effects down.  It also ignores `static`s and `const`s,
because they cannot contain method calls.

Where the ambiguity leads to errors in type inference, you can use the
`type_ascription` nightly feature to disambiguate.

Now you can call functions expecting `String`, `Cow<str>`, and all other types
that implement `From<&str>` with a string literal:

```rust
struct Person {
    first: String,
    last: String,
    birthplace: Cow<str>,
}

process_persons(&[
    Person { first: "Doug", last: "Piranha", birthplace: "London" },
    Person { first: "Dinsdale", last: "Piranha", birthplace: "London" },
]);
```
