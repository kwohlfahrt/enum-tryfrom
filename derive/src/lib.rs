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
            if ident == "TryFromPrimitiveType" {
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
                Ok(#name::#ident)
            }
        }
    });
    let mut tokens = quote::Tokens::new();
    tokens.append_separated(blocks, "else");

    quote! {
        impl TryFrom<#ty> for #name {
            type Error = enum_tryfrom::InvalidEnumValue;

            fn try_from(v: #ty) -> Result<Self, Self::Error> {
                #tokens else {
                    Err(Self::Error::new())
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
            panic!("`TryFromPrimitive` is only supported on Enums")
        };
    let impls = types.map(|ty| impl_single_type(name, &ty, variants.iter()));

    let mut tokens = quote::Tokens::new();
    tokens.append_all(impls);
    tokens
}

/// Generate `TryFrom` for each primitive type mentioned as a
/// `TryFromPrimitiveType` attribute.
///
/// When combined with unwrap, this is as fast as a bounds check + transmute, at
/// least for small enums. See the `benches` folder for benchmarks.
///
/// # Examples
///
/// ```
/// #![feature(try_from)]
/// use std::convert::TryFrom;
///
/// extern crate enum_tryfrom;
///
/// #[macro_use] extern crate enum_tryfrom_derive;
/// #[derive(PartialEq,Debug,TryFromPrimitive)]
/// #[TryFromPrimitiveType="u32"]
/// enum Foo {
///     FirstFoo = 1,
///     SecondFoo,
///     ThirdFoo,
/// }
///
/// # fn main() {
/// let v : u32 = 2;
/// assert_eq!(Foo::try_from(v).unwrap(), Foo::SecondFoo);
/// # }
/// ```
#[proc_macro_derive(TryFromPrimitive, attributes(TryFromPrimitiveType))]
pub fn from_primitive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_from_primitive(&ast);

    gen.parse().unwrap()
}
