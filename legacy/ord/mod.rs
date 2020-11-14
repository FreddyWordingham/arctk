//! Ordering module.

pub mod key;
pub mod list;
pub mod name;
pub mod register;
pub mod set;
pub mod sort;

pub use self::{key::*, list::*, name::*, register::*, set::*, sort::*};
