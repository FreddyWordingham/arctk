//! Camera builder structure.

use crate::{
    err::Error,
    file::Build,
    geom::{Camera, Orient},
    math::Pos3,
};
use arctk_attr::load;
use std::path::Path;

/// Loadable camera structure.
#[load]
pub struct CameraBuilder {
    /// Position.
    pos: Pos3,
    /// Target.
    tar: Pos3,
    /// Horizontal field-of-view [deg].
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Optional super-sampling power.
    super_sampling: Option<usize>,
}

impl Build for CameraBuilder {
    type Inst = Camera;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let ss = if let Some(ss) = self.super_sampling {
            ss
        } else {
            1
        };

        Ok(Self::Inst::new(
            Orient::new_tar(self.pos, &self.tar),
            self.fov.to_radians(),
            self.res,
            ss,
        ))
    }
}
