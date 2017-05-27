#[macro_use]
extern crate enum_derive;

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
    assert_eq!(Foo::from(1), Foo::FirstFoo);
}

#[test]
fn test_var() {
    let v : u32 = 2;
    assert_eq!(Foo::from(v), Foo::SecondFoo);
    let v : i32 = 2;
    assert_eq!(Foo::from(v), Foo::SecondFoo);
}

#[test]
#[should_panic]
fn test_out_of_bounds() {
    Foo::from(4);
}
