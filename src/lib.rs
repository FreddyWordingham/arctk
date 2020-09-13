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
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::modulo_arithmetic,
    clippy::module_name_repetitions,
    clippy::panic,
    clippy::print_stdout,
    clippy::unreachable,
    clippy::unwrap_used
)]

// pub mod geom;
// pub mod math;
// pub mod sim;
pub mod data;
pub mod err;
pub mod file;
pub mod img;
pub mod meta;
pub mod phys;
pub mod tools;
pub mod util;

pub use self::{data::*, err::*, file::*, img::*, meta::*, phys::*, tools::*, util::*};
