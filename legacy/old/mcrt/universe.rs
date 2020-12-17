//! Simulation input conglomerate.

use super::{Attributes, Material, Settings};
use crate::{
    geom::{Grid, Mesh, Tree},
    ord::{Key, Set},
};

/// MCRT simulation resources conglomerate.
pub struct Universe<'a> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, &'a Key>,
    /// Regular grid.
    pub grid: &'a Grid,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Key, Mesh>,
    /// Attributes.
    pub attrs: &'a Set<Key, Attributes>,
    /// Materials.
    pub mats: &'a Set<Key, Material>,
}

impl<'a> Universe<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Tree<'a, &Key>,
        grid: &'a Grid,
        sett: &'a Settings,
        surfs: &'a Set<Key, Mesh>,
        attrs: &'a Set<Key, Attributes>,
        mats: &'a Set<Key, Material>,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            surfs,
            attrs,
            mats,
        }
    }
}
