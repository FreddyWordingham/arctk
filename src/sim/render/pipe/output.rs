//! Output data structure.

use crate::{
    err::Error,
    file::Save,
    img::{Gradient, Image},
    ord::{X, Y},
    report,
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::{ops::AddAssign, path::Path};

/// MCRT output data.
pub struct Output<'a> {
    /// Flight distances.
    pub dist: Array2<f64>,
    /// Render time.
    pub time: Array2<f64>,
    /// Colouring gradient.
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
            dist: Array2::zeros(res),
            time: Array2::zeros(res),
            grad,
        }
    }
}

impl<'a> AddAssign<&Self> for Output<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.dist += &rhs.dist;
        self.time += &rhs.time;
    }
}

impl<'a> Save for Output<'a> {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let max_dist = self.dist.max()?;
        report!("Maximum distance", max_dist, "m");
        Image::new(self.dist.map(|x| self.grad.get((*x / max_dist) as f32)))
            .save(&out_dir.join("distance.png"))?;

        let max_time = self.time.max()?;
        report!("Maximum time", max_time, "ms");
        Image::new(self.time.map(|x| self.grad.get((*x / max_time) as f32)))
            .save(&out_dir.join("time.png"))?;

        Ok(())
    }
}
