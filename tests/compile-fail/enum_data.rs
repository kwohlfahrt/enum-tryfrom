#[macro_use]
extern crate enum_proc_derive;

#[derive(FromPrimitive)]
//~^  ERROR proc-macro derive panicked
#[FromPrimitiveType="u32"]
enum Foo {
    Bar(String),
    Baz(i32),
}

fn main() {}
