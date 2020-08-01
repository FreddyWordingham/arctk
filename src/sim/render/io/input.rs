//! Rendering simulation input structure.

use crate::{
    display_field, display_field_ln,
    grid::Grid,
    render::{Attributes, Settings},
    tree::Cell,
    Mesh, Set,
};
use palette::{Gradient, LinSrgba};
use std::fmt::{Display, Formatter, Result};

/// Cartographer mapping input structure.
pub struct Input<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Attributes.
    pub attrs: &'a Set<Attributes>,
    /// Colours.
    pub cols: &'a Set<Gradient<LinSrgba>>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        attrs: &'a Set<Attributes>,
        cols: &'a Set<Gradient<LinSrgba>>,
    ) -> Self {
        Self {
            tree,
            sett,
            surfs,
            attrs,
            cols,
        }
    }
}

impl<'a> Display for Input<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "tree", &self.tree)?;
        display_field_ln!(fmt, "settings", &self.sett)?;
        display_field_ln!(fmt, "surfaces", &self.surfs)?;
        display_field_ln!(fmt, "attributes", &self.attrs)?;
        display_field!(fmt, "colours", &format!("total {}", self.cols.map().len()))
    }
}
