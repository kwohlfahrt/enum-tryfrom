# Enum TryFrom

[![Build Status](https://travis-ci.org/kwohlfahrt/enum-tryfrom.svg?branch=master)](https://travis-ci.org/kwohlfahrt/enum-tryfrom)

A crate to derive the `TryFrom` trait on enums using procedural macros.

Expect frequent breakage for now.

## Documentation

This adds the ability to `#[derive(FromPrimitive)]`, which in combination with
one (well zero, but that's not useful) or more `#[FromPrimitiveType="type"]`
attributes lets you automatically derive `From<type>` for enum types.

Detailed documentation can be generated with `cargo doc`.
