//! Input module.

use crate::{display_field_ln, grid::Grid, tree::Cell, Mesh, Set};
use std::fmt::{Display, Formatter, Result};

/// Cartographer mapping input structure.
pub struct Input<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Surface tree.
    pub grid: &'a Grid,
    // /// Settings.
    // pub sett: &'a Technical,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        grid: &'a Grid,
        // sett: &'a Technical,
        surfs: &'a Set<Mesh>,
    ) -> Self {
        Self {
            tree,
            grid,
            // sett,
            surfs,
        }
    }
}

impl<'a> Display for Input<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "tree", &self.tree)?;
        display_field_ln!(fmt, "grid", &self.grid)?;
        // display_field_ln!(fmt, "settings", &self.sett)?;
        display_field_ln!(fmt, "surfaces", &self.surfs)
    }
}
