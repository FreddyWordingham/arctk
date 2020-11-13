//! Optical material.

use super::Local;
use crate::{access, math::Formula};

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
    pub fn env(&self, w: f64) -> Local {
        let index = self.ref_index.y(w);

        let scat = self.scat_coeff.y(w);

        let abs = if let Some(abs_coeff_formula) = &self.abs_coeff {
            abs_coeff_formula.y(w)
        } else {
            0.0
        };

        let shift = if let Some(shift_coeff_formula) = &self.shift_coeff {
            shift_coeff_formula.y(w)
        } else {
            0.0
        };

        let g = self.asym_fact.y(w);

        Local::new(index, scat, abs, shift, g)
    }
}
