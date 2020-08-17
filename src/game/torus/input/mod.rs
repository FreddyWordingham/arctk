//! Input sub-module.

pub mod symbols;
pub mod system;
pub mod world;

pub use self::{symbols::*, system::*, world::*};

/// Torus game input structure.
pub struct Input<'a> {
    /// System settings.
    pub sys: &'a System,
    /// World settings.
    pub world: &'a World,
    /// Symbols settings.
    pub symbols: &'a Symbols,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sys: &'a System, world: &'a World, symbols: &'a Symbols) -> Self {
        Self {
            sys,
            world,
            symbols,
        }
    }
}
