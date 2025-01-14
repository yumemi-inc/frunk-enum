# Frunk Enum Support

> **Warning**  
> This fork was for supporting frunk v0.4 and won't be maintained anymore.
> See https://github.com/Metaswitch/frunk-enum/pull/31 .

[![Crates.io - frunk-enum](https://img.shields.io/crates/v/frunk-enum-core.svg)](https://crates.io/crates/frunk-enum-core) [![Build Status](https://travis-ci.org/Metaswitch/frunk-enum.svg?branch=master)](https://travis-ci.org/Metaswitch/frunk-enum) [![License: MIT/APACHE-2.0](https://img.shields.io/badge/License-MIT%2FAPACHE--2.0-green.svg)](https://opensource.org/licenses/MIT)

## Usage

These crates augment the [frunk](https://docs.rs/frunk/latest/frunk/) crate to
allow transmogrification of `enum`s as well as `struct`s.  For more information
on transmogrification and the `LabelledGeneric` trait it's based around, see
https://docs.rs/frunk/latest/frunk/#transmogrifying and
https://docs.rs/frunk/latest/frunk/labelled/trait.LabelledGeneric.html.

To take advantage of this feature for your own enum you need to:

* Add the `frunk-enum-derive` crate to your `Cargo.toml`
* Mark the enum with the custom derive:

    ```
    #[derive(LabelledGenericEnum)]
    enum Foo {
        Bar(String),
        Baz { name: String, id: Identity },
    }
    ```
* Add the `frunk-enum-core` and `frunk-core` crates to your `Cargo.toml`
* Then (assuming there's a `NewFoo` enum with the same structure as `Foo`) you can write:

    ```
    let foo = Foo::Baz { name: "Andy".into(), id: Identity };
    let new_foo: NewFoo = foo.transmogrify();
    ```

This works by deriving an implementation of `LabelledGeneric` for `Foo` which
allows conversion to and from an instance of a generic sum-type.  The core
crate provides tools for converting between these generic sum-types where the
bodies of the variants are recursively transmogrifiable.  This allows for
arbitrarily deep conversion between types, especially useful where the two
types in question are autogenerated from some common input file, but are deemed
to be different types by the rust compiler (e.g. because they're in separate
crates).

## Contributing

Thank you for your interest in frunk-enum, all contributions are welcome whether
they be reports of issues, feature requests or code contributions.

### Issue Contribution

When opening issues please provide a description of the issue, the version of
frunk and frunk-enum plus example code which exhibits the issue.

### Code Contribution

If you wish to contribute code to frunk-enum, you are welcome to do so using a
github pull request, however you will need to sign each commit, to indicate
that you are abiding to the terms of the [DCO](DCO).
