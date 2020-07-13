//! Gradient form implementation.

use crate::{display_field, Build, Error};
use attr::load;
use palette::LinSrgba;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable colour gradient structure.
#[load]
pub struct Gradient(
    /// List of colours.
    Vec<String>,
);

impl Build for Gradient {
    type Inst = palette::Gradient<LinSrgba>;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut cols = Vec::with_capacity(self.0.len());

        for col in self.0 {
            let col_arr = hex::decode(col.replace("#", ""))?;

            let r = f32::from(col_arr[0]) / 255.0;
            let g = f32::from(col_arr[1]) / 255.0;
            let b = f32::from(col_arr[2]) / 255.0;
            let a = f32::from(col_arr[3]) / 255.0;

            cols.push(LinSrgba::new(r, g, b, a));
        }

        Ok(Self::Inst::new(cols))
    }
}

impl Display for Gradient {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field!(fmt, "number of colours", self.0.len())
    }
}
