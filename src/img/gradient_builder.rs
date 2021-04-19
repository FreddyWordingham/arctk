//! Gradient builder implementation.

use crate::{
    img::{Colour, Gradient},
    ord::Build,
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Loadable colour gradient structure.
#[file]
pub struct GradientBuilder(
    /// List of colours.
    Vec<String>,
);

impl Build for GradientBuilder {
    type Inst = Gradient;

    #[inline]
    fn build(self) -> Self::Inst {
        let mut cols = Vec::with_capacity(self.0.len());

        for col in self.0 {
            let col_arr = hex::decode(col.replace("#", ""))
                .unwrap_or_else(|_| panic!("Failed to parse hexidecimal string: {}.", col));

            let r = f32::from(col_arr[0]) / 255.0;
            let g = f32::from(col_arr[1]) / 255.0;
            let b = f32::from(col_arr[2]) / 255.0;
            let a = f32::from(col_arr[3]) / 255.0;

            cols.push(Colour::new(r, g, b, a));
        }

        Self::Inst::new(cols)
    }
}

impl Display for GradientBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.0.is_empty() {
            write!(fmt, "[]")?;
        } else {
            write!(fmt, "[{}", self.0[0])?;
            for col in self.0.iter().skip(1) {
                write!(fmt, ", {}", col)?;
            }
            write!(fmt, "]")?;
        }
        Ok(())
    }
}
