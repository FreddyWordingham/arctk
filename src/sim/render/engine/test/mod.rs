//! Rendering engine: Test.

pub mod event;
pub mod fast;
pub mod live;
pub mod output;
pub mod paint;

pub use self::{event::*, fast::*, live::*, output::*, paint::*};
