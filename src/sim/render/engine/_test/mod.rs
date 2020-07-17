//! Rendering engine: Test.

pub mod event;
pub mod fast;
pub mod illumination;
pub mod live;
pub mod output;
pub mod paint;

pub use self::{event::*, fast::*, illumination::*, live::*, output::*, paint::*};
