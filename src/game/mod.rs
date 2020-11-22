//! Game module.

pub mod comp;
pub mod ent;
pub mod parts;
pub mod state;
pub mod sys;

pub use self::{comp::*, ent::*, parts::*, state::*, sys::*};
