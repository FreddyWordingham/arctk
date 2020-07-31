//! Light surface structure.

use crate::{
    display_field, display_field_ln, mcrt::Light, Build, Error, MeshForm, ProbabilityForm,
};
use attr::load;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Loadable light structure.
#[load]
pub struct LightForm {
    /// Object path link.
    surf: MeshForm,
    /// Emission form.
    spec: ProbabilityForm,
}

impl Build for LightForm {
    type Inst = Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mesh = self.surf.build(in_dir)?;
        let spec = self.spec.build(in_dir)?;

        Ok(Self::Inst::new(mesh, spec))
    }
}

impl Display for LightForm {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "surface path", &self.surf)?;
        display_field!(fmt, "spectrum form", &self.spec)
    }
}
