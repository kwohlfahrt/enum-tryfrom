#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate enum_proc_derive;

#[derive(Debug)]
struct InvalidEnumValue(());

#[derive(PartialEq,Eq,Debug,FromPrimitive)]
#[FromPrimitiveType="u32"]
#[FromPrimitiveType="i32"]
enum Foo {
    FirstFoo = 1,
    SecondFoo,
    ThirdFoo,
}

#[test]
fn test_literal() {
    assert_eq!(Foo::try_from(1).unwrap(), Foo::FirstFoo);
}

#[test]
fn test_var() {
    let v : u32 = 2;
    assert_eq!(Foo::try_from(v).unwrap(), Foo::SecondFoo);
    let v : i32 = 2;
    assert_eq!(Foo::try_from(v).unwrap(), Foo::SecondFoo);
}

#[test]
fn test_out_of_bounds() {
    assert!(Foo::try_from(4).is_err());
}
