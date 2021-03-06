# Enum TryFrom

[![Build Status](https://travis-ci.org/kwohlfahrt/enum-tryfrom.svg?branch=master)](https://travis-ci.org/kwohlfahrt/enum-tryfrom)

A crate to derive the `TryFrom` trait on enums using procedural macros.

Expect frequent breakage for now.

## Documentation

[![Documentation](https://docs.rs/enum-tryfrom/badge.svg)](https://docs.rs/enum-tryfrom)

This adds the ability to `#[derive(TryFromPrimitive)]`, which in combination
with one (well zero, but that's not useful) or more
`#[FromPrimitiveType="type"]` attributes lets you automatically derive
`TryFrom<type>` for enum types.

Detailed documentation can be generated with `cargo doc`.
