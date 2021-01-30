//! Material builder.

use crate::{fmt_report, math::FormulaBuilder, ord::Build, phys::Material};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

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
    fn build(self) -> Self::Inst {
        let ref_index = self.ref_index.build();
        let scat_coeff = self.scat_coeff.build();
        let abs_coeff = self.abs_coeff.map(Build::build);
        let shift_coeff = self.shift_coeff.map(Build::build);
        let asym_fact = self.asym_fact.build();

        Self::Inst::new(ref_index, scat_coeff, abs_coeff, shift_coeff, asym_fact)
    }
}

impl Display for MaterialBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.ref_index, "refractive index");
        fmt_report!(fmt, self.scat_coeff, "scattering coefficient (m^-1)");

        let abs_coeff = if let Some(ref abs_coeff) = self.shift_coeff {
            format!("{}", abs_coeff)
        } else {
            "NONE".to_string()
        };
        fmt_report!(fmt, abs_coeff, "absorption coefficient (m^-1)");

        let shift_coeff = if let Some(ref shift_coeff) = self.shift_coeff {
            format!("{}", shift_coeff)
        } else {
            "NONE".to_string()
        };
        fmt_report!(fmt, shift_coeff, "shift coefficient (m^-1)");

        fmt_report!(fmt, self.asym_fact, "asymmetry factor");
        Ok(())
    }
}
