//! Support library of attribute macros.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(clippy::implicit_return, clippy::module_name_repetitions)]

extern crate proc_macro;
extern crate proc_macro2;

mod input;
mod load;
mod output;
mod save;

use proc_macro::TokenStream;

/// Create the attribute macro input.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn input(metadata: TokenStream, input: TokenStream) -> TokenStream {
    input::implementation(&metadata, input)
}

/// Create the attribute macro output.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn output(metadata: TokenStream, input: TokenStream) -> TokenStream {
    output::implementation(&metadata, input)
}

/// Create the attribute macro load.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn load(metadata: TokenStream, input: TokenStream) -> TokenStream {
    load::implementation(&metadata, input)
}

/// Create the attribute macro save.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn save(metadata: TokenStream, input: TokenStream) -> TokenStream {
    save::implementation(&metadata, input)
}
