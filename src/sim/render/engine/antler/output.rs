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

        let path = out_dir.join(&format!("{}_{}", time, "image.png"));
        println!("Saving: {}", path.display());
        self.image.save(&path)?;

        let time_max = *self.time.max()?;
        let time_img = self.time.map(|x| self.scale.get(x.log(time_max) as f32));
        let path = out_dir.join(&format!("{}_{}", time, "log_time.png"));
        println!("Saving: {}", path.display());
        time_img.save(&path)
    }
}
