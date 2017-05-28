# Enum Proc Derive

[![Build Status](https://travis-ci.org/kwohlfahrt/enum-proc-derive.svg?branch=master)](https://travis-ci.org/kwohlfahrt/enum-proc-derive)

A crate to derive useful enum attributes using procedural macros.

Expect frequent breakage for now.

## Documentation

This adds the ability to `#[derive(FromPrimitive)]`, which in combination with
one (well zero, but that's not useful) or more `#[FromPrimitiveType="type"]`
attributes lets you automatically derive `From<type>` for enum types.

Detailed documentation can be generated with `cargo doc`.
