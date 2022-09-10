//! Domain partitioning.

pub mod surface;
pub mod surface_builder;
pub mod tree;
pub mod tree_builder;

pub use self::{surface::*, surface_builder::*, tree::*, tree_builder::*};
