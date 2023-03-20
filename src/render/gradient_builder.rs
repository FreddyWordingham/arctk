//! Gradient builder.

use palette::{Gradient, LinSrgba};
use serde::Deserialize;

/// Colour gradient.
#[derive(Deserialize)]
pub struct GradientBuilder(
    /// List of colours.
    Vec<String>,
);

impl GradientBuilder {
    /// Build the colour gradient instance.
    #[inline]
    #[must_use]
    pub fn build(self) -> Gradient<LinSrgba> {
        let mut cols = Vec::with_capacity(self.0.len());

        for col in self.0 {
            let col_arr = hex::decode(col.replace('#', ""))
                .unwrap_or_else(|_| panic!("Failed to parse hexadecimal string: {col}."));

            let r = f32::from(col_arr[0]) / 255.0;
            let g = f32::from(col_arr[1]) / 255.0;
            let b = f32::from(col_arr[2]) / 255.0;
            let a = f32::from(col_arr[3]) / 255.0;

            cols.push(LinSrgba::new(r, g, b, a));
        }

        Gradient::new(cols)
    }
}
