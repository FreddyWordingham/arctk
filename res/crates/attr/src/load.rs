//! Implementation function of the load attribute macro.

use proc_macro::TokenStream;

/// Create the attribute macro load.
#[inline]
#[must_use]
pub fn implementation(_metadata: &TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Deserialize, proc::Load)]
        #input
    };
    output.into()
}
