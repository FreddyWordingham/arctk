//! Implementation function of the save attribute macro.

use proc_macro::TokenStream;

/// Create the attribute macro save.
#[inline]
#[must_use]
pub fn implementation(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, proc::Save)]
        #input
    };
    output.into()
}
