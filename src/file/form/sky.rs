//! Sky builder structure.

use crate::{display_field, display_field_ln, Build, Error, Pos3};
use attr::load;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Scene settings structure.
#[load]
pub struct Sky {
    /// Sky brightness fraction.
    brightness: f64,
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [deg].
    sun_rad: f64,
}

impl Build for Sky {
    type Inst = crate::render::Sky;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.brightness,
            self.sun_pos,
            self.sun_rad.to_radians(),
        ))
    }
}

impl Display for Sky {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "sky brightness", self.brightness)?;
        display_field_ln!(fmt, "sun position", &self.sun_pos, "m")?;
        display_field!(fmt, "sun radius", self.sun_rad, "deg")
    }
}
