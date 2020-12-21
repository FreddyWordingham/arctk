//! Simulation input.

use crate::{
    fmt_report,
    geom::{Grid, Tree},
    ord::{Register, Set},
    sim::cartographer::{Attribute, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// Cartographer simulation resources conglomerate.
pub struct Input<'a> {
    /// Material register.
    pub mat_reg: &'a Register,
    /// Attributes.
    pub attrs: &'a Set<Attribute>,
    /// Hit-scan tree.
    pub tree: &'a Tree<'a, Attribute>,
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
        mat_reg: &'a Register,
        attrs: &'a Set<Attribute>,
        tree: &'a Tree<'a, Attribute>,
        grid: &'a Grid,
        sett: &'a Settings,
    ) -> Self {
        Self {
            mat_reg,
            attrs,
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
        fmt_report!(fmt, self.mat_reg, "material register");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.tree, "hit-scan tree");
        fmt_report!(fmt, self.grid, "measurement grid");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
