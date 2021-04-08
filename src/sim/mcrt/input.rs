//! Simulation input.

use crate::{
    fmt_report,
    geom::{Grid, Tree},
    math::{Formula, Pos3},
    ord::{Register, Set},
    phys::{Light, Material},
    sim::mcrt::{Attribute, Settings},
    util::fmt::Analyze,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// MCRT simulation resources conglomerate.
pub struct Input<'a> {
    /// Spectrometer register.
    pub spec_reg: &'a Register,
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
    /// Optional fluorophore properties (concentration map, absorption spectra).
    pub shifts_conc_spec: &'a Option<(Array3<f64>, Formula)>,
    /// Optional camera position.
    pub cam_pos: &'a Option<Pos3>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        spec_reg: &'a Register,
        mats: &'a Set<Material>,
        attrs: &'a Set<Attribute>,
        light: &'a Light,
        tree: &'a Tree<Attribute>,
        grid: &'a Grid,
        sett: &'a Settings,
        shifts_conc_spec: &'a Option<(Array3<f64>, Formula)>,
        cam_pos: &'a Option<Pos3>,
    ) -> Self {
        Self {
            spec_reg,
            mats,
            attrs,
            light,
            tree,
            grid,
            sett,
            shifts_conc_spec,
            cam_pos,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.spec_reg, "spectrometer register");
        fmt_report!(fmt, self.attrs, "materials");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, self.tree, "hit-scan tree");
        fmt_report!(fmt, self.grid, "measurement grid");
        fmt_report!(fmt, self.sett, "settings");
        if let Some(shifts_conc_spec) = self.shifts_conc_spec {
            fmt_report!(fmt, shifts_conc_spec.0.display(), "Fluorophore map");
            fmt_report!(fmt, shifts_conc_spec.1, "Fluorophore absorption spectra");
        }
        if let Some(cam_pos) = self.cam_pos {
            fmt_report!(fmt, cam_pos, "Camera position");
        }
        Ok(())
    }
}
