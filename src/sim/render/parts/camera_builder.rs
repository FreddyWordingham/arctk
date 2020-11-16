//! Camera builder structure.

use crate::{err::Error, file::Build, sim::render::Camera};
use arctk_attr::load;
use std::path::Path;

/// Loadable camera structure.
#[load]
pub struct CameraBuilder {}

impl Build for CameraBuilder {
    type Inst = Camera;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new())
    }
}
