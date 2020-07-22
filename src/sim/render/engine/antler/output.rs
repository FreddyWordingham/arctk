//! Output structure.

use crate::{Error, Image, Save, X, Y};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Intensity colouring scale.
    scale: Gradient<LinSrgba>,
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
    pub fn new(img_res: [usize; 2], scale: Gradient<LinSrgba>) -> Self {
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            scale,
            image: Image::default(img_res),
            time: Array2::zeros(img_res),
            first_hit_index: Array2::zeros(img_res),
        }
    }

    /// Create a weighted scan circle.
    #[inline]
    #[must_use]
    fn scan_circle(rad: i32) -> Vec<([i32; 2], f64)> {
        debug_assert!(rad > 0);

        let rad_sq = rad.pow(2);

        let mut scan = Vec::with_capacity(((rad * 2) + 1).pow(2) as usize);
        let mut total_weight = 0.0;
        for xi in -rad..=rad {
            for yi in -rad..=rad {
                // Not central pixel.
                if xi == 0 && yi == 0 {
                    continue;
                }

                // Sample circle, not square.
                let r_sq = xi.pow(2) + yi.pow(2);
                if r_sq > rad_sq {
                    continue;
                }

                let w = 1.0 - (f64::from(r_sq).sqrt() / f64::from(rad));
                total_weight += w;

                scan.push(([xi, yi], w));
            }
        }
        scan.shrink_to_fit();

        for (_, w) in &mut scan {
            *w /= total_weight;
        }

        scan
    }

    /// Calculate the edge map from the first hit index data.
    #[inline]
    fn find_edges<T: PartialEq>(img: &Array2<T>, rad: i32) -> Result<Array2<f64>, Error> {
        debug_assert!(rad > 0);

        let [width, height] = [img.shape()[0] as i32, img.shape()[1] as i32];
        let mut edges = Vec::with_capacity(img.len());

        let scan = Self::scan_circle(rad);
        for xi in 0..width {
            for yi in 0..height {
                let mut max = 0.0;
                let mut diff = 0.0;
                let centre = &img[[xi as usize, yi as usize]];

                for ([dx, dy], w) in &scan {
                    let px = xi + dx;
                    let py = yi + dy;

                    if (px >= 0) && (px < width) && (py >= 0) && (py < height) {
                        max += w;
                        if centre != &img[[px as usize, py as usize]] {
                            diff += w;
                        }
                    }
                }

                edges.push(diff / max);
            }
        }

        println!("Edges len: {}", edges.len());

        Ok(Array2::from_shape_vec(
            [width as usize, height as usize],
            edges,
        )?)
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

        let first_hit_index_edge = Self::find_edges(&self.first_hit_index, 10)?;
        let edge_max = first_hit_index_edge.max()? + 1.0e-3;
        let first_hit_index_edge_img =
            ((&first_hit_index_edge + 1.0e-3) / edge_max).map(|x| self.scale.get(*x as f32));
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
