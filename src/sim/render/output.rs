//! Output data structure.

use crate::{
    err::Error,
    fmt_report,
    fs::Save,
    img::{Colour, Gradient, Image},
    ord::{X, Y},
    report,
    util::fmt::datacube::display_datasquare,
    util::gradient::to_string,
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Formatter};
use std::{ops::AddAssign, path::Path};

/// Rendering output data.
pub struct Output<'a> {
    /// Render time.
    pub time: Array2<f64>,
    /// Lighting factors.
    pub light: Array2<f64>,
    /// Shadowing factors.
    pub shadow: Array2<f64>,
    /// Colouring.
    pub colour: Image,
    /// Data colouring gradient.
    grad: &'a Gradient,
}

impl<'a> Output<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2], grad: &'a Gradient) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            time: Array2::zeros(res),
            light: Array2::zeros(res),
            shadow: Array2::zeros(res),
            colour: Image::new_blank(res, Colour::default()),
            grad,
        }
    }
}

impl<'a> AddAssign<&Self> for Output<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.time += &rhs.time;
        self.light += &rhs.light;
        self.shadow += &rhs.shadow;
        self.colour += &rhs.colour;
    }
}

impl<'a> Save for Output<'a> {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        let max_time = *self.time.max()?;
        report!("Maximum time", max_time, "ms");
        Image::new(self.time.map(|x| self.grad.get(x.log(max_time) as f32)))
            .save(&out_dir.join("time.png"))?;

        let max_light = self.light.max()?;
        report!("Maximum light value", max_light);
        Image::new(self.light.map(|x| self.grad.get((*x / max_light) as f32)))
            .save(&out_dir.join("light.png"))?;

        let max_shadow = self.shadow.max()?;
        report!("Maximum shadow value", max_shadow);
        Image::new(self.shadow.map(|x| self.grad.get((*x / max_shadow) as f32)))
            .save(&out_dir.join("shadow.png"))?;

        self.colour.save(&out_dir.join("colour.png"))
    }
}

impl Display for Output<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        writeln!(fmt, "Time data...")?;
        display_datasquare(fmt, &self.time)?;
        writeln!(fmt, "Light data...")?;
        display_datasquare(fmt, &self.light)?;
        writeln!(fmt, "Shadow data...")?;
        display_datasquare(fmt, &self.shadow)?;
        fmt_report!(fmt, to_string(self.grad, 32), "gradient");
        Ok(())
    }
}
