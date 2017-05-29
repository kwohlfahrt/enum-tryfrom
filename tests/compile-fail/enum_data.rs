#[macro_use]
extern crate enum_tryfrom_derive;

#[derive(TryFromPrimitive)]
//~^  ERROR proc-macro derive panicked
#[TryFromPrimitiveType="u32"]
enum Foo {
    Bar(String),
    Baz(i32),
}

fn main() {}
