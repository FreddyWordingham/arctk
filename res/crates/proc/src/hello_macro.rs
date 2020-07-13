//! Implementation function of the procedural macro `HelloMacro`.

use crate::proc_macro::TokenStream;
use quote::quote;

/// Add a function which reports the name of the type it is bound to.
#[inline]
#[must_use]
pub fn derive_impl(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello! I am a {} type.", stringify!(#name));
            }
        }
    };
    gen.into()
}
