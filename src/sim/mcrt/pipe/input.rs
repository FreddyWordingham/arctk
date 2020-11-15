//! Simulation input.

use crate::{
    geom::{Grid, Tree},
    opt::{Light, Material},
    sim::mcrt::{Attribute, Settings},
};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Hit-scan tree.
    pub tree: &'a Tree<'a>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings,
    /// Materials.
    pub mats: &'a Vec<Material>,
    /// Attributes.
    pub attrs: &'a Vec<Attribute>,
    /// Emission light.
    pub light: &'a Light,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Tree,
        grid: &'a Grid,
        sett: &'a Settings,
        mats: &'a Vec<Material>,
        attrs: &'a Vec<Attribute>,
        light: &'a Light,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            mats,
            attrs,
            light,
        }
    }
}
