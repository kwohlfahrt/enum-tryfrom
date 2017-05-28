//! This crate contains macros for deriving useful traits on C-like enums

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

fn impl_single_type<'a, I: Iterator<Item=&'a syn::Variant>>
    (name: &syn::Ident, ty: &syn::Ident, variants: I) -> quote::Tokens {
    let blocks = variants.map(|var| {
        let ident = &var.ident;
        if var.data != syn::VariantData::Unit {
            panic!("Enum variant may not store data!")
        }
        quote!{
            if v == #name::#ident as #ty {
                #name::#ident
            }
        }
    });
    let mut tokens = quote::Tokens::new();
    tokens.append_separated(blocks, "else");

    quote! {
        impl From<#ty> for #name {
            fn from(v: #ty) -> Self {
                #tokens else {
                    panic!("Unexpected enum variant!")
                }
            }
        }
    }
}

fn impl_from_primitive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let types = ast.attrs.iter().filter_map(filter_primitive_type_attr)
        .map(syn::Ident::new);
    let variants =
        if let syn::Body::Enum(ref variants) = ast.body {
            variants
        } else {
            panic!("`FromPrimitive` is only supported on Enums")
        };
    let impls = types.map(|ty| impl_single_type(name, &ty, variants.iter()));

    let mut tokens = quote::Tokens::new();
    tokens.append_all(impls);
    tokens
}

/// Generate `From` for each primitive type mentioned as a `FromPrimitiveType`
/// attribute.
///
/// # Panics
///
/// Generated `impl` panics if primitive values are not valid enum values.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate enum_derive;
/// #[derive(PartialEq,Debug,FromPrimitive)]
/// #[FromPrimitiveType="u32"]
/// enum Foo {
///     FirstFoo = 1,
///     SecondFoo,
///     ThirdFoo,
/// }
///
/// # fn main() {
/// let v : u32 = 2;
/// assert_eq!(Foo::from(v), Foo::SecondFoo);
/// # }
/// ```
#[proc_macro_derive(FromPrimitive, attributes(FromPrimitiveType))]
pub fn from_primitive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_from_primitive(&ast);

    gen.parse().unwrap()
}
