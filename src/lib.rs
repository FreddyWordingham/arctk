//! Library core.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::as_conversions,
    clippy::blanket_clippy_restriction_lints,
    clippy::cargo_common_metadata,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::panic,
    clippy::pub_use,
    clippy::self_named_module_files,
    clippy::separated_literal_suffix,
    clippy::single_char_lifetime_names
)]

pub mod rt;
