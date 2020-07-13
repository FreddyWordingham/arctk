//! Support library of procedural macros.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::implicit_return,
    clippy::module_name_repetitions,
    clippy::panic,
    clippy::result_expect_used,
    clippy::wildcard_enum_match_arm
)]

extern crate proc_macro;
extern crate proc_macro2;

mod hello_macro;
mod input;
mod load;
mod output;
mod save;

use crate::proc_macro::TokenStream;

/// Create the procedural macro `HelloMacro`.
#[proc_macro_derive(HelloMacro)]
#[inline]
#[must_use]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Unable to parse token stream.");
    hello_macro::derive_impl(&ast)
}

/// Create the procedural macro Input.
#[proc_macro_derive(Input)]
#[inline]
#[must_use]
pub fn input_derive(input: TokenStream) -> TokenStream {
    input::input_derive_impl(input)
}

/// Create the procedural macro Load.
#[proc_macro_derive(Load)]
#[inline]
#[must_use]
pub fn load_derive(input: TokenStream) -> TokenStream {
    load::load_derive_impl(input)
}

/// Create the procedural macro Output.
#[proc_macro_derive(Output)]
#[inline]
#[must_use]
pub fn output_derive(input: TokenStream) -> TokenStream {
    output::output_derive_impl(input)
}

/// Create the procedural macro Save.
#[proc_macro_derive(Save)]
#[inline]
#[must_use]
pub fn save_derive(input: TokenStream) -> TokenStream {
    save::save_derive_impl(input)
}
