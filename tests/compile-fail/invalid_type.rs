#[macro_use]
extern crate enum_tryfrom_derive;

#[derive(FromPrimitive)]
//~^  ERROR proc-macro derive panicked
struct Foo {
    foo: i32,
}

fn main() {}
