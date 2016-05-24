# overload-strings

This is a quick and silly syntax extension, mostly to familiarize myself with
compiler plugins using custom attributes.  It does the equivalent of Haskell's
`OverloadedStrings`, i.e. it inserts an `.into()` call onto every string
literal.  No extra trait is necessary.

NB: Currently this breaks statics containing string literals, because methods
can't be called there.  Don't annotate whole modules when they contain such
statics.

## Usage

As a compiler plugin, requires nightly Rust.  Add
```
#![feature(plugin)]
#![plugin(overload_strings)]
```
then apply the `#[overload_strings]` attribute on the item(s) you want to
overload string literals in (module, fn, impl, ...).

Where the ambiguity leads to errors in type inference, you can use the
`type_ascription` nightly feature to disambiguate.

## TODO

* Handle statics/consts.
* Don't insert `into()` when type ascription dictates `&str`.
