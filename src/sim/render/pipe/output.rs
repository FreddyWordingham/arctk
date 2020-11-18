//! Output data structure.

use crate::{
    err::Error,
    file::Save,
    img::{Colour, Gradient, Image},
    math::Vec3,
    ord::{X, Y},
    report,
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::{ops::AddAssign, path::Path};

/// Rendering output data.
pub struct Output<'a> {
    /// Flight distances.
    pub dist: Array2<f64>,
    /// Render time.
    pub time: Array2<f64>,
    /// Final surface normal.
    pub final_norm: Array2<Vec3>,
    /// Block colouring.
    pub block_colour: Image,
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
            final_norm: Array2::default(res),
            block_colour: Image::new_blank(res, Colour::default()),
            grad,
        }
    }
}

impl<'a> AddAssign<&Self> for Output<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.dist += &rhs.dist;
        self.time += &rhs.time;
        self.final_norm += &rhs.final_norm;
        self.block_colour += &rhs.block_colour;
    }
}

impl<'a> Save for Output<'a> {
    #[inline]
    fn save_data(&self, out_dir: &Path) -> Result<(), Error> {
        let max_dist = self.dist.max()?;
        report!("Maximum distance", max_dist, "m");
        Image::new(self.dist.map(|x| self.grad.get((*x / max_dist) as f32)))
            .save(&out_dir.join("distance.png"))?;

        let max_time = self.time.max()?;
        report!("Maximum time", max_time, "ms");
        Image::new(self.time.map(|x| self.grad.get((*x / max_time) as f32)))
            .save(&out_dir.join("time.png"))?;

        Image::new(
            self.final_norm
                .map(|n| n.normalize())
                .map(|n| Colour::new(n.x.abs() as f32, n.y.abs() as f32, n.z.abs() as f32, 1.0)),
        )
        .save(&out_dir.join("normals.png"))?;
        // Image::new(self.map(|x| self.grad.get((*x / max_time) as f32)))
        //     .save(&out_dir.join("time.png"))?;

        self.block_colour.save(&out_dir.join("block_colour.png"))
    }
}
