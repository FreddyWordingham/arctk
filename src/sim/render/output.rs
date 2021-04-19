//! Output data structure.

use crate::{
    err::Error,
    fmt_report,
    fs::Save,
    img::{Colour, Gradient, Image},
    ord::{X, Y},
    report,
    util::{fmt::DataSquare, gradient::to_string},
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

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
    /// Image number.
    img_id: usize,
}

impl<'a> Output<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2], grad: &'a Gradient, img_id: usize) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            time: Array2::zeros(res),
            light: Array2::zeros(res),
            shadow: Array2::zeros(res),
            colour: Image::new_blank(res, Colour::default()),
            grad,
            img_id,
        }
    }
}

unsafe impl<'a> Sync for Output<'a> {}

impl<'a> AddAssign<&Self> for Output<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!(self.img_id == rhs.img_id);

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
        report!(max_time, "Maximum time", "ms");
        Image::new(self.time.map(|x| self.grad.get(x.log(max_time) as f32)))
            .save(&out_dir.join(&format!("time_{:04}.png", self.img_id)))?;

        let max_light = self.light.max()?;
        report!(max_light, "Maximum light value");
        Image::new(self.light.map(|x| self.grad.get((*x / max_light) as f32)))
            .save(&out_dir.join(&format!("light_{:04}.png", self.img_id)))?;

        let max_shadow = self.shadow.max()?;
        report!(max_shadow, "Maximum shadow value");
        Image::new(self.shadow.map(|x| self.grad.get((*x / max_shadow) as f32)))
            .save(&out_dir.join(&format!("shadow_{:04}.png", self.img_id)))?;

        self.colour
            .save(&out_dir.join(&format!("colour_{:04}.png", self.img_id)))
    }
}

impl Display for Output<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, "Time data", DataSquare::new(&self.time));
        fmt_report!(fmt, "Light data", DataSquare::new(&self.light));
        fmt_report!(fmt, "Shadow data", DataSquare::new(&self.shadow));
        fmt_report!(fmt, to_string(self.grad, 32), "gradient");
        Ok(())
    }
}
