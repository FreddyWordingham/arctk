//! Output structure.

use crate::{Dir3, Error, Image, Save, Vec3, X, Y};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Base image.
    pub image: Image,
    /// Time data.
    pub time: Array2<f64>,
    /// First hit.
    pub first_hit: Array2<usize>,
    /// Distance data.
    pub first_dist: Array2<f64>,
    /// First norm.
    pub first_norm: Array2<Vec3>,
    /// Last hit.
    pub last_hit: Array2<usize>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(img_res: [usize; 2]) -> Self {
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            image: Image::default(img_res),
            time: Array2::zeros(img_res),
            first_hit: Array2::zeros(img_res),
            first_dist: Array2::zeros(img_res),
            first_norm: Array2::default(img_res),
            last_hit: Array2::zeros(img_res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.time += &rhs.time;
        self.first_hit += &rhs.first_hit;
        self.first_dist += &rhs.first_dist;
        self.first_norm += &rhs.first_norm;
        self.last_hit += &rhs.last_hit;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();

        let greyscale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
        ]);
        let red_scale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 0.0, 0.0, 1.0),
        ]);
        let green_scale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(0.0, 1.0, 0.0, 1.0),
        ]);
        let blue_scale = Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(0.0, 0.0, 1.0, 1.0),
        ]);

        let path = out_dir.join(&format!("{}_{}", time, "image.png"));
        println!("Saving: {}", path.display());
        self.image.save(&path)?;

        let time_max = *self.time.max()?;
        let time_log = self.time.map(|x| x.log(time_max));
        let time_log_img = time_log.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "time_log.png"));
        println!("Saving: {}", path.display());
        time_log_img.save(&path)?;

        let time_lin = &self.time / time_max;
        let time_lin_img = time_lin.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "time_lin.png"));
        println!("Saving: {}", path.display());
        time_lin_img.save(&path)?;

        let first_hit_max = *self.first_hit.max()? as f64 + 1.0e-3;
        let first_hit_lin = self.first_hit.map(|x| *x as f64 + 1.0e-3) / first_hit_max;
        let first_hit_img = first_hit_lin.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_hit.png"));
        println!("Saving: {}", path.display());
        first_hit_img.save(&path)?;

        let first_dist_max = *self.first_dist.max()?;
        let first_dist = &self.first_dist / first_dist_max;
        let first_dist_img = first_dist.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_dist_lin.png"));
        println!("Saving: {}", path.display());
        first_dist_img.save(&path)?;

        let first_dist_log = self
            .first_dist
            .map(|d| 1.0 - (first_dist_max - d).log(first_dist_max));
        let first_dist_log_img = first_dist_log.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "first_dist_log.png"));
        println!("Saving: {}", path.display());
        first_dist_log_img.save(&path)?;

        let first_norm_img = self.first_norm.map(|v| {
            if v.magnitude() < 1.0e-3 {
                greyscale.get(1.0)
            } else {
                let n = Dir3::new_normalize(*v);
                red_scale.get(n.x as f32) + green_scale.get(n.y as f32) + blue_scale.get(n.z as f32)
            }
        });
        let path = out_dir.join(&format!("{}_{}", time, "first_norm.png"));
        println!("Saving: {}", path.display());
        first_norm_img.save(&path)?;

        let last_hit_max = *self.last_hit.max()? as f64 + 1.0e-3;
        let last_hit_lin = self.last_hit.map(|x| *x as f64 + 1.0e-3) / last_hit_max;
        let last_hit_img = last_hit_lin.map(|x| greyscale.get(*x as f32));
        let path = out_dir.join(&format!("{}_{}", time, "last_hit.png"));
        println!("Saving: {}", path.display());
        last_hit_img.save(&path)
    }
}
