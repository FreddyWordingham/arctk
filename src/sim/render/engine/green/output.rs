//! Output structure.

use crate::{Error, Image, Save, X, Y};
use ndarray::Array2;
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Base image.
    pub image: Image,
    /// Distance data.
    pub dist: Array2<f64>,
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
            dist: Array2::zeros(img_res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.dist += &rhs.dist;
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
        self.image.save(&path)

        // let path = out_dir.join(&format!("{}_{}", time, "dist.png"));
        // println!("Saving: {}", path.display());
        // self.dist.save(&path)
    }
}
