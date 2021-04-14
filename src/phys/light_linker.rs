//! Light surface structure.

use crate::{
    err::Error,
    fmt_report,
    geom::Emitter,
    math::Probability,
    ord::{Link, Name, Set},
    phys::{Light, Material},
};
use std::fmt::{Display, Formatter};

/// Photon emission structure linker.
pub struct LightLinker {
    /// Power [J/s].
    power: f64,
    /// Emitter.
    emitter: Emitter,
    /// Emission spectrum.
    spec: Probability,
    /// Emitting material.
    mat: Name,
}

impl LightLinker {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(power: f64, emitter: Emitter, spec: Probability, mat: Name) -> Self {
        debug_assert!(power > 0.0);

        Self {
            power,
            emitter,
            spec,
            mat,
        }
    }
}

impl<'a> Link<'a, Material> for LightLinker {
    type Inst = Light<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        vec![self.mat.clone()]
    }

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        let mat = mats
            .get(&self.mat)
            .unwrap_or_else(|| panic!("Failed to link light-material key: {}", &self.mat));
        let power = self.power;
        let emitter = self.emitter;
        let spec = self.spec;

        Ok(Self::Inst::new(power, emitter, spec, mat))
    }
}

impl Display for LightLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.power, "power (J/s)");
        fmt_report!(fmt, self.emitter, "emitter");
        fmt_report!(fmt, self.spec, "emission spectrum");
        fmt_report!(fmt, self.mat, "emission material");
        Ok(())
    }
}
