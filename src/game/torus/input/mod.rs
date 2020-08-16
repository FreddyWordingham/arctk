//! Input sub-module.

pub mod system;
pub mod world;

pub use self::system::*;
pub use self::world::*;

/// Torus game input structure.
pub struct Input<'a> {
    /// System settings.
    pub sys: &'a System,
    /// World settings.
    pub world: &'a World,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sys: &'a System, world: &'a World) -> Self {
        Self { sys, world }
    }
}
