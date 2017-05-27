#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn filter_primitive_type_attr(attr: &syn::Attribute) -> Option<&str> {
    use syn::MetaItem::NameValue;

    match attr.value {
        NameValue(ref ident, syn::Lit::Str(ref value, _)) => {
            if ident == "FromPrimitiveType" {
                Some(value)
            } else {
                None
            }
        },
        _ => None,
    }
}

fn impl_from_primitive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let mut attrs = ast.attrs.iter().filter_map(filter_primitive_type_attr);
    let ty = syn::Ident::new(attrs.next().expect(
        "Missing `FromPrimitiveType` annotation!"
    ));
    quote! {
        impl From<#ty> for #name {
            fn from(v: #ty) -> Self {
                #name::FirstFoo
            }
        }
    }
}

#[proc_macro_derive(FromPrimitive, attributes(FromPrimitiveType))]
pub fn from_primitive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_from_primitive(&ast);

    gen.parse().unwrap()
}
