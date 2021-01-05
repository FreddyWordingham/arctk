//! Optical material.

use crate::{access, fmt_report, math::Formula, phys::Local};

/// Optical properties.
pub struct Material {
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient [1/m].
    scat_coeff: Formula,
    /// Absorption coefficient [1/m].
    abs_coeff: Option<Formula>,
    /// Shifting coefficient [1/m].
    shift_coeff: Option<Formula>,
    /// Asymmetry factor.
    asym_fact: Formula,
}

impl Material {
    access!(ref_index, Formula);
    access!(scat_coeff, Formula);
    access!(abs_coeff, Option<Formula>);
    access!(shift_coeff, Option<Formula>);
    access!(asym_fact, Formula);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Option<Formula>,
        shift_coeff: Option<Formula>,
        asym_fact: Formula,
    ) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym_fact,
        }
    }

    /// Generate an optical environment for a given wavelength.
    #[inline]
    #[must_use]
    pub fn sample_environment(&self, w: f64) -> Local {
        let index = self.ref_index.y(w);

        let scat = self.scat_coeff.y(w);

        let abs = self
            .abs_coeff
            .as_ref()
            .map_or(0.0, |abs_coeff_formula| abs_coeff_formula.y(w));

        let shift = self
            .shift_coeff
            .as_ref()
            .map_or(0.0, |shift_coeff_formula| shift_coeff_formula.y(w));
        let g = self.asym_fact.y(w);

        Local::new(index, scat, abs, shift, g)
    }
}

impl Display for Material {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.ref_index, "refractive index");
        fmt_report!(fmt, self.scat_coeff, "scattering coefficient (m^-1)");

        let abs_coeff = if let Some(abs_coeff) = self.shift_coeff {
            format!("{}", abs_coeff)
        } else {
            "NONE".to_string()
        };
        fmt_report!(fmt, self.shift_coeff, "absorption coefficient (m^-1)");

        let shift_coeff = if let Some(shift_coeff) = self.shift_coeff {
            format!("{}", shift_coeff)
        } else {
            "NONE".to_string()
        };
        fmt_report!(fmt, self.shift_coeff, "shift coefficient (m^-1)");

        fmt_report!(fmt, self.asym_fact, "asymmetry factor");
        Ok(())
    }
}
