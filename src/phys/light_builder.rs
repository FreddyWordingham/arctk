//! Light surface structure.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::EmitterBuilder,
    math::ProbabilityBuilder,
    phys::Light,
};
use arctk_attr::load;
use std::path::Path;

/// Loadable light structure.
#[load]
pub struct LightBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    emit: EmitterBuilder,
    /// Emission form.
    spec: Redirect<ProbabilityBuilder>,
}

impl Build for LightBuilder {
    type Inst = Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let emit = self.emit.build(in_dir)?;
        let spec = self.spec.build(in_dir)?.build(in_dir)?;

        Ok(Self::Inst::new(self.power, emit, spec))
    }
}
