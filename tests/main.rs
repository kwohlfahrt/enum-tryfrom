#[macro_use]
extern crate enum_derive;

#[derive(PartialEq,Eq,Debug,FromPrimitive)]
#[FromPrimitiveType="u32"]
enum Foo {
    FirstFoo = 0,
    SecondFoo,
    ThirdFoo,
}

#[test]
fn test_literal() {
    assert_eq!(Foo::from(10), Foo::FirstFoo);
}

#[test]
fn test_var() {
    let v : u32 = 10;
    assert_eq!(Foo::from(v), Foo::FirstFoo);
}
