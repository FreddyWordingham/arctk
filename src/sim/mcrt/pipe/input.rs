//! Simulation input.

use crate::{
    geom::{Grid, Tree},
    ord::Set,
    sim::mcrt::{Attribute, Light, Material, Settings},
};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Materials.
    pub mats: &'a Set<Material>,
    /// Attributes.
    pub attrs: &'a Set<Attribute<'a>>,
    /// Emission light.
    pub light: &'a Light,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, Attribute<'a>>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings<'a>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        mats: &'a Set<Material>,
        attrs: &'a Set<Attribute>,
        light: &'a Light,
        tree: &'a Tree<Attribute>,
        grid: &'a Grid,
        sett: &'a Settings,
    ) -> Self {
        Self {
            mats,
            attrs,
            light,
            sett,
            grid,
            tree,
        }
    }
}
