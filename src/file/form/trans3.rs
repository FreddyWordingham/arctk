//! Transform form implementation.

use crate::{display_field, display_field_ln, Build, Error, Vec3};
use attr::load;
use nalgebra::{Translation3, UnitQuaternion};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable transform structure.
#[load]
#[derive(Clone)]
pub struct Trans3 {
    /// Optional translation to apply.
    trans: Option<Translation3<f64>>,
    /// Rotation applied as Euler angles.
    rot: Option<Vec3>,
    /// Optional uniform scaling to apply.
    scale: Option<f64>,
}

impl Build for Trans3 {
    type Inst = crate::Trans3;

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

impl Display for Trans3 {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        if let Some(trans) = self.trans {
            display_field_ln!(fmt, "translation", trans)?
        } else {
            display_field_ln!(fmt, "translation", "none")?
        };
        if let Some(rot) = self.rot {
            display_field_ln!(fmt, "rotation", rot)?
        } else {
            display_field_ln!(fmt, "rotation", "none")?
        };
        if let Some(scale) = self.scale {
            display_field!(fmt, "scaling", scale)
        } else {
            display_field!(fmt, "scaling", "none")
        }
    }
}
