//! Input module.

use crate::{
    cartographer::Settings, display_field, display_field_ln, grid::Grid, tree::Cell, Group, Mesh,
    Set,
};
use std::fmt::{Display, Formatter, Result};

/// Cartographer mapping input structure.
pub struct Input<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Surface tree.
    pub grid: &'a Grid,
    /// Settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Interfaces.
    pub inters: &'a Set<(Group, Group)>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        grid: &'a Grid,
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        inters: &'a Set<(Group, Group)>,
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

impl<'a> Display for Input<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "tree", &self.tree)?;
        display_field_ln!(fmt, "grid", &self.grid)?;
        display_field_ln!(fmt, "settings", &self.sett)?;
        display_field_ln!(fmt, "surfaces", &self.surfs)?;
        display_field!(fmt, "total interfaces", self.inters.map().len())
    }
}
