//! Simulation input.

use crate::{
    fmt_report,
    geom::{Grid, Tree},
    ord::Set,
    phys::{Light, Material},
    sim::mcrt::{Attribute, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Materials.
    pub mats: &'a Set<Material>,
    /// Attributes.
    pub attrs: &'a Set<Attribute<'a>>,
    /// Emission light.
    pub light: &'a Light<'a>,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, Attribute<'a>>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings,
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
            tree,
            grid,
            sett,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.attrs, "materials");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, self.tree, "hit-scan tree");
        fmt_report!(fmt, self.grid, "measurement grid");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
