//! Output structure.

use crate::{Error, Image, Save, X, Y};
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
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.time += &rhs.time;
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
        time_lin_img.save(&path)
    }
}
