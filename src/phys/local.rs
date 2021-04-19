//! Local optical environment.

use crate::clone;

/// Localised optical environment properties.
pub struct Local {
    /// Refractive index.
    ref_index: f64,
    /// Scattering coefficient. [m^-1]
    scat_coeff: f64,
    /// Absorption coefficient. [m^-1]
    abs_coeff: f64,
    /// Shift coefficient. [m^-1]
    shift_coeff: f64,
    /// Asymmetry parameter.
    asym: f64,
}

impl Local {
    clone!(ref_index, f64);
    clone!(scat_coeff, f64);
    clone!(abs_coeff, f64);
    clone!(shift_coeff, f64);
    clone!(asym, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        ref_index: f64,
        scat_coeff: f64,
        abs_coeff: f64,
        shift_coeff: f64,
        asym: f64,
    ) -> Self {
        debug_assert!(ref_index >= 1.0);
        debug_assert!(scat_coeff > 0.0);
        debug_assert!(abs_coeff >= 0.0);
        debug_assert!(shift_coeff >= 0.0);
        debug_assert!(asym.abs() <= 1.0);

        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym,
        }
    }

    /// Calculate the interaction coefficient.
    #[inline]
    #[must_use]
    pub fn inter_coeff(&self) -> f64 {
        self.scat_coeff + self.abs_coeff + self.shift_coeff
    }

    /// Calculate the albedo.
    #[inline]
    #[must_use]
    pub fn albedo(&self) -> f64 {
        1.0 - (self.abs_coeff / self.inter_coeff())
    }

    /// Calculate the shifting probability.
    #[inline]
    #[must_use]
    pub fn shift_prob(&self) -> f64 {
        self.shift_coeff / self.inter_coeff()
    }
}
