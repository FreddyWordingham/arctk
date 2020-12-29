//! Camera builder structure.

use crate::{
    fmt_report,
    geom::{Camera, Orient},
    math::{Pos3, Vec3},
    ord::{Build, X, Y},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Loadable camera structure.
#[file]
#[derive(Clone)]
pub struct CameraBuilder {
    /// Position.
    pos: Pos3,
    /// Target.
    tar: Pos3,
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Optional super-sampling power.
    ss_power: Option<usize>,
}

impl CameraBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3, fov: f64, res: [usize; 2], ss_power: Option<usize>) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(ss_power.is_none() || ss_power.unwrap() > 1);

        Self {
            pos,
            tar,
            fov,
            res,
            ss_power,
        }
    }

    /// Move the camera.
    #[inline]
    pub fn travel(&mut self, d: Vec3) {
        self.pos += d;
    }
}

impl Build for CameraBuilder {
    type Inst = Camera;

    #[inline]
    fn build(self) -> Self::Inst {
        Camera::new(
            Orient::new_tar(self.pos, &self.tar),
            self.fov.to_radians(),
            self.res,
            self.ss_power.map_or(1, |ss| ss),
        )
    }
}

impl Display for CameraBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.pos.x, self.pos.y, self.pos.z),
            "position (m)"
        );
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.tar.x, self.tar.y, self.tar.z),
            "target (m)"
        );
        fmt_report!(fmt, self.fov, "field of view (deg)");
        fmt_report!(
            fmt,
            &format!("[{} x {}]", self.res[X], self.res[Y]),
            "resolution"
        );

        let ss_power = if let Some(n) = self.ss_power {
            format!("{} sub-samples", n * n)
        } else {
            "OFF".to_string()
        };
        fmt_report!(fmt, ss_power, "super sampling");

        Ok(())
    }
}
