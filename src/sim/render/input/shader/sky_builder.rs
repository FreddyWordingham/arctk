//! Sky builder structure.

use crate::{display_field, display_field_ln, render::Sky, Build, Error, PerlinMap, Pos3, X, Y};
use attr::load;
use rand::thread_rng;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Scene settings structure.
#[load]
pub struct SkyBuilder {
    /// Sky brightness fraction.
    brightness: f64,
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [deg].
    sun_rad: f64,
    /// Noise segments.
    segments: [usize; 2],
}

impl Build for SkyBuilder {
    type Inst = Sky;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.brightness,
            self.sun_pos,
            self.sun_rad.to_radians(),
            PerlinMap::new(self.segments, &mut thread_rng()),
        ))
    }
}

impl Display for SkyBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "sky brightness", self.brightness)?;
        display_field_ln!(fmt, "sun position", &self.sun_pos, "m")?;
        display_field_ln!(fmt, "sun radius", self.sun_rad, "deg")?;
        display_field!(
            fmt,
            "segments",
            &format!("{} x {}", self.segments[X], self.segments[Y])
        )
    }
}
