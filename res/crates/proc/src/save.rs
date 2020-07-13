//! Implementation function of the save procedural macro.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// Implement `Save` trait using json parsing.
#[inline]
#[must_use]
pub fn save_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let name = match input {
        Item::Struct(s) => s.ident,
        Item::Enum(e) => e.ident,
        Item::Union(u) => u.ident,
        _ => panic!("Can not derive json for this item."),
    };

    let output = quote! {
        impl crate::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) -> std::result::Result<(), crate::Error> {
                crate::as_json(self, path)
            }
        }
    };

    TokenStream::from(output)
}
