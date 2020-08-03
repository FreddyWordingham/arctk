//! Output structure.

use crate::{Error, Image, Save, X, Y};
use std::{ops::AddAssign, path::Path};

/// Rendering output structure.
pub struct Output {
    /// Coloured output image.
    pub img: Image,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            img: Image::new(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // // Get current time string.
        // let time = chrono::offset::Local::now()
        //     .format("%Y%m%d%H%M%S")
        //     .to_string();
        // let path = out_dir.join(time);
        // std::fs::create_dir(&path)?;
        let path = out_dir;

        let p = path.join("img.png");
        println!("Saving: {}", p.display());
        self.img.save(&p)
    }
}
