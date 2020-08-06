//! Light surface structure.

use crate::{
    display_field, display_field_ln, mcrt::Light, Build, Error, MeshBuilder, ProbabilityForm,
};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable light structure.
#[load]
pub struct LightBuilder {
    /// Power [J/s].
    power: f64,
    /// Object path link.
    surf: MeshBuilder,
    /// Emission form.
    spec: ProbabilityForm,
}

impl Build for LightBuilder {
    type Inst = Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mesh = self.surf.build(in_dir)?;
        let spec = self.spec.build(in_dir)?;

        Ok(Self::Inst::new(self.power, mesh, spec))
    }
}

impl Display for LightBuilder {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "power", self.power, "J/s")?;
        display_field_ln!(fmt, "surface path", &self.surf)?;
        display_field!(fmt, "spectrum form", &self.spec)
    }
}
