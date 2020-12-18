//! Transform form implementation.

use crate::{err::Error, fs::Build, math::Vec3};
use arctk_attr::load;
use nalgebra::{Translation3, UnitQuaternion};
use std::path::Path;

/// Loadable transform structure.
#[load]
#[derive(Clone)]
pub struct Trans3Builder {
    /// Optional translation to apply.
    trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    rot: Option<Vec3>,
    /// Optional uniform scaling to apply.
    scale: Option<f64>,
}

impl Build for Trans3Builder {
    type Inst = crate::math::Trans3;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let trans = self
            .trans
            .unwrap_or_else(|| Translation3::new(0.0, 0.0, 0.0));

        let rot = self.rot.unwrap_or_else(|| Vec3::new(0.0, 0.0, 0.0));
        let rot = UnitQuaternion::from_euler_angles(
            rot.x.to_radians(),
            rot.y.to_radians(),
            rot.z.to_radians(),
        );

        let scale = self.scale.unwrap_or(1.0);

        Ok(Self::Inst::from_parts(trans, rot, scale))
    }
}
