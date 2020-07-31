//! Light surface structure.

use crate::{access, display_field, display_field_ln, mcrt::Photon, Emit, Mesh, ProbabilityForm};
use attr::load;
use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

/// Loadable light structure.
#[load]
pub struct LightForm {
    /// Object path link.
    surf: PathBuf,
    /// Emission form.
    spec: ProbabilityForm,
}

impl<'a> Display for LightForm {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "surface path", self.surf.display())?;
        display_field!(fmt, "spectrum form", &self.spec)
    }
}
