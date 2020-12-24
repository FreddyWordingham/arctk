//! Camera builder structure.

use crate::{
    fmt_report,
    geom::{Camera, Orient},
    math::Pos3,
    ord::{Build, X, Y, Z},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Loadable camera structure.
#[file]
pub struct CameraBuilder {
    /// Position.
    pos: [f64; 3],
    /// Target.
    tar: [f64; 3],
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Optional super-sampling power.
    ss_power: Option<usize>,
}

impl Build for CameraBuilder {
    type Inst = Camera;

    #[inline]
    fn build(self) -> Self::Inst {
        Camera::new(
            Orient::new_tar(
                Pos3::new(self.pos[X], self.pos[Y], self.pos[Z]),
                &Pos3::new(self.tar[X], self.tar[Y], self.tar[Z]),
            ),
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
            &format!("({}, {}, {})", self.pos[X], self.pos[Y], self.pos[Z]),
            "position (m)"
        );
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.tar[X], self.tar[Y], self.tar[Z]),
            "target (m)"
        );
        fmt_report!(fmt, self.fov, "field of view (deg)");
        fmt_report!(
            fmt,
            &format!("[{}x{}]", self.res[X], self.res[Y]),
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
