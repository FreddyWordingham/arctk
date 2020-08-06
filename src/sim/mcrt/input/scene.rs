//! MCRT simulation input structure.

use crate::{
    display_field, display_field_ln,
    mcrt::{Attributes, Material, Settings},
    tree::Cell,
    Mesh, Set,
};
use std::fmt::{Display, Formatter, Result};

/// MCRT main input structure.
pub struct Scene<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Attributes.
    pub attrs: &'a Set<Attributes>,
    /// Materials.
    pub mats: &'a Set<Material>,
}

impl<'a> Scene<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        attrs: &'a Set<Attributes>,
        mats: &'a Set<Material>,
    ) -> Self {
        Self {
            tree,
            sett,
            surfs,
            attrs,
            mats,
        }
    }
}

impl<'a> Display for Scene<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "tree", &self.tree)?;
        display_field_ln!(fmt, "settings", &self.sett)?;
        display_field_ln!(fmt, "surfaces", &self.surfs)?;
        display_field_ln!(fmt, "attributes", &self.attrs)?;
        display_field!(fmt, "materials", &self.mats)
    }
}
