//! Ordering module.

pub mod list;
pub mod name;
pub mod register;
pub mod set;
pub mod sort;

pub use self::{list::*, name::*, register::*, set::*, sort::*};
