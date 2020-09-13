//! Gradient builder implementation.

use crate::{Build, Error};
use arctk_attr::load;
use palette::LinSrgba;
use std::path::Path;

/// Loadable colour gradient structure.
#[load]
pub struct GradientBuilder(
    /// List of colours.
    Vec<String>,
);

impl Build for GradientBuilder {
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
