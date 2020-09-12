//! Camera-builder implementation.

use crate::{
    display_field_ln,
    render::{Focus, Lens, Sensor},
    AspectRatio, Build, Error, Pos3, X, Y,
};
use arctk_attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable camera builder structure.
#[load]
pub struct CameraBuilder {
    /// Camera position [m].
    pos: Pos3,
    /// Target position [m].
    tar: Pos3,
    /// Optional depth-of-field samples and maximum angular sample [deg].
    dof: Option<(i32, f64)>,
    /// Optional targeting swivel adjustment [deg].
    swivel: Option<[f64; 2]>,
    /// Horizontal field of view [deg].
    hr_fov: f64,
    /// Aspect ratio.
    aspect: AspectRatio,
    /// Horizontal pixel resolution.
    hr_res: u64,
    /// Optional super-sampling power.
    ss_power: Option<i32>,
}

impl Build for CameraBuilder {
    type Inst = super::Camera;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let dof = if let Some((samples, angle)) = self.dof {
            Some((samples, angle.to_radians()))
        } else {
            None
        };
        let focus = Focus::new(self.pos, self.tar, dof);

        let swivel = if let Some(s) = self.swivel {
            s
        } else {
            [0.0, 0.0]
        };
        let lens = Lens::new(
            [swivel[X].to_radians(), swivel[Y].to_radians()],
            self.hr_fov.to_radians(),
        );
        let sensor = Sensor::new(&self.aspect, self.hr_res, self.ss_power);

        Ok(Self::Inst::new(focus, lens, sensor))
    }
}

impl Display for CameraBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "position", &self.pos, "m")?;
        display_field_ln!(fmt, "target", &self.tar, "m")?;
        if let Some((dof_samples, dof_angle)) = self.dof {
            display_field_ln!(fmt, "depth-of-field samples", dof_samples)?;
            display_field_ln!(fmt, "depth-of-field angle", dof_angle, "deg")?;
        } else {
            display_field_ln!(fmt, "depth-of-field", "[OFF]")?;
        }
        if let Some(s) = self.swivel {
            display_field_ln!(fmt, "swivel", &format!("{:.2}, {:.2}", s[X], s[Y]), "deg")?;
        } else {
            display_field_ln!(fmt, "swivel", "[OFF]")?;
        }
        display_field_ln!(fmt, "field-of-view", self.hr_fov, "deg")?;
        display_field_ln!(fmt, "aspect ratio", &self.aspect)?;
        display_field_ln!(fmt, "horizontal resolution", self.hr_res)?;
        if let Some(ss) = self.ss_power {
            display_field_ln!(fmt, "super-sampling power", ss)?;
        } else {
            display_field_ln!(fmt, "super-sampling", "[OFF]")?;
        }
        Ok(())
    }
}
