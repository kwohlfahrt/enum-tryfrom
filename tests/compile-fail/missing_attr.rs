#[macro_use]
extern crate enum_derive;

#[derive(FromPrimitive)]
//~^  ERROR proc-macro derive panicked
enum Foo {
    FirstFoo,
}

fn main() {}
