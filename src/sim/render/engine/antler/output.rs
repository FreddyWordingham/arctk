//! Output structure.

use crate::{render::img, Error, Image, Save, X, Y};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Intensity colouring scale.
    scale: Gradient<LinSrgba>,
    /// Blackening scale.
    black_scale: Gradient<LinSrgba>,
    /// Outline scale.
    outline_scale: Gradient<LinSrgba>,
    /// Base image.
    pub image: Image,
    /// Time data.
    pub time: Array2<f64>,
    /// First hit index.
    pub first_hit_index: Array2<i32>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        img_res: [usize; 2],
        scale: Gradient<LinSrgba>,
        black_scale: Gradient<LinSrgba>,
        outline_scale: Gradient<LinSrgba>,
    ) -> Self {
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            scale,
            black_scale,
            outline_scale,
            image: Image::default(img_res),
            time: Array2::zeros(img_res),
            first_hit_index: Array2::zeros(img_res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.time += &rhs.time;
        self.first_hit_index += &rhs.first_hit_index;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // Get current time string.
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();

        // Save main image.
        let path = out_dir.join(&format!("{}_{}", time, "image.png"));
        println!("Saving: {}", path.display());
        self.image.save(&path)?;

        // Save first hit data.
        let first_hit_max = f64::from(*self.first_hit_index.max()? + 1) + 1.0e-3;
        let first_hit_lin =
            self.first_hit_index.map(|x| f64::from(*x + 1) + 1.0e-3) / first_hit_max;
        let first_hit_img = first_hit_lin.map(|x| self.scale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_hit.png"));
        println!("Saving: {}", path.display());
        first_hit_img.save(&path)?;

        let first_hit_index_edge = img::find_edges(&self.first_hit_index, 3)?;
        let edge_max = first_hit_index_edge.max()? + 1.0e-3;
        let first_hit_index_edge_lin = (&first_hit_index_edge + 1.0e-3) / edge_max;
        let first_hit_index_edge_lin = first_hit_index_edge_lin.map(|x| x.sqrt());
        let first_hit_index_edge_img =
            first_hit_index_edge_lin.map(|x| self.black_scale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_hit_edge_outline.png"));
        println!("Saving: {}", path.display());
        first_hit_index_edge_img.save(&path)?;
        let first_hit_index_edge_img =
            first_hit_index_edge_lin.map(|x| self.outline_scale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_hit_edge.png"));
        println!("Saving: {}", path.display());
        first_hit_index_edge_img.save(&path)?;

        // Save temporal data.
        let time_max = *self.time.max()?;
        let time_img = self.time.map(|x| self.scale.get(x.log(time_max) as f32));
        let path = out_dir.join(&format!("{}_{}", time, "log_time.png"));
        println!("Saving: {}", path.display());
        time_img.save(&path)
    }
}
