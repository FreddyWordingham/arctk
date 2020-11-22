//! Game module.

pub mod comp;
pub mod ent;
pub mod state;
pub mod sys;

pub use self::{comp::*, ent::*, state::*, sys::*};
