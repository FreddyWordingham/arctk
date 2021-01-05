//! Material builder.

use crate::{err::Error, math::FormulaBuilder, ord::Build, phys::Material};
use arctk_attr::file;
use std::path::Path;

/// Loadable material.
#[file]
pub struct MaterialBuilder {
    /// Refractive index.
    ref_index: FormulaBuilder,
    /// Scattering coefficient [1/m].
    scat_coeff: FormulaBuilder,
    /// Absorption coefficient [1/m].
    abs_coeff: Option<FormulaBuilder>,
    /// Shifting coefficient [1/m].
    shift_coeff: Option<FormulaBuilder>,
    /// Asymmetry factor.
    asym_fact: FormulaBuilder,
}

impl Build for MaterialBuilder {
    type Inst = Material;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let ref_index = self.ref_index.build(in_dir)?;
        let scat_coeff = self.scat_coeff.build(in_dir)?;
        let abs_coeff = if let Some(abs_coeff) = self.abs_coeff {
            Some(abs_coeff.build(in_dir)?)
        } else {
            None
        };
        let shift_coeff = if let Some(shift_coeff) = self.shift_coeff {
            Some(shift_coeff.build(in_dir)?)
        } else {
            None
        };
        let asym_fact = self.asym_fact.build(in_dir)?;

        Ok(Self::Inst::new(
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym_fact,
        ))
    }
}
