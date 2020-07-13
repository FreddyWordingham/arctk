//! Implementation function of the load procedural macro.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// Implement `Load` trait using json parsing.
#[inline]
#[must_use]
pub fn load_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let (name, generics) = match input {
        Item::Struct(s) => (s.ident, s.generics),
        Item::Enum(e) => (e.ident, e.generics),
        Item::Union(u) => (u.ident, u.generics),
        _ => panic!("Can not derive json for this item."),
    };

    let output = quote! {
        impl crate::Load<#generics> for #name<#generics> {
            #[inline]
            fn load(path: &std::path::Path) -> std::result::Result<Self, crate::Error> {
                crate::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}
