//! Mapping simulation input structure.

use super::{Interface, Settings};
use crate::{
    geom::{Grid, Mesh, Tree},
    ord::{Key, Set},
};

/// Rendering main input structure.
pub struct Landscape<'a> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, &'a Key>,
    /// Sampling grid.
    pub grid: &'a Grid,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Key, Mesh>,
    /// Interfaces.
    pub inters: &'a Set<Key, Interface>,
}

impl<'a> Landscape<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Tree<'a, &Key>,
        grid: &'a Grid,
        sett: &'a Settings,
        surfs: &'a Set<Key, Mesh>,
        inters: &'a Set<Key, Interface>,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            surfs,
            inters,
        }
    }
}
