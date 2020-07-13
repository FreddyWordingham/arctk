//! Implementation function of the input attribute macro.

use proc_macro::TokenStream;

/// Create the attribute macro input.
#[inline]
#[must_use]
pub fn implementation(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Deserialize, proc::Input)]
        #input
    };
    output.into()
}
