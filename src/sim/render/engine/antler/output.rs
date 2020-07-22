//! Output structure.

use crate::{render::img, Error, Image, Save, X, Y};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgba};
use std::{ops::AddAssign, path::Path};

/// First hit edge detection radius.
const FIRST_HIT_RAD: i32 = 4;
/// Last hit edge detection radius.
const LAST_HIT_RAD: i32 = 2;

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
    /// Las hit index.
    pub last_hit_index: Array2<i32>,
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
            last_hit_index: Array2::zeros(img_res),
        }
    }

    /// Save the main image data.
    #[inline]
    fn save_main_img(&self, path: &Path) -> Result<(), Error> {
        let p = path.join("image.png");
        println!("Saving: {}", p.display());
        self.image.save(&p)
    }

    /// Save the temporal image data.
    #[inline]
    fn save_time_img(&self, path: &Path) -> Result<(), Error> {
        let max = *self.time.max()?;
        let img = self.time.map(|x| self.scale.get(x.log(max) as f32));
        let p = path.join("time.png");
        println!("Saving: {}", p.display());
        img.save(&p)
    }

    /// Save the first hit data.
    #[inline]
    fn save_first_hit_index(&self, path: &Path) -> Result<(), Error> {
        {
            let max = f64::from(*self.first_hit_index.max()? + 1) + 1.0e-3;
            let linear = self.first_hit_index.map(|x| f64::from(*x + 1) + 1.0e-3) / max;

            let img = linear.map(|x| self.scale.get(*x as f32));
            let p = path.join("first_hit.png");
            println!("Saving: {}", p.display());
            img.save(&p)?;
        }

        {
            let edges = img::find_edges(&self.first_hit_index, FIRST_HIT_RAD)?;
            let max = edges.max()? + 1.0e-3;
            let linear = edges.map(|x| *x + 1.0e-3) / max;

            {
                let img = linear.map(|x| self.black_scale.get(*x as f32));
                let p = path.join("first_hit_edges_bk.png");
                println!("Saving: {}", p.display());
                img.save(&p)?;
            }

            {
                let img = linear.map(|x| self.outline_scale.get(*x as f32));
                let p = path.join("first_hit_edges_ol.png");
                println!("Saving: {}", p.display());
                img.save(&p)
            }
        }
    }

    /// Save the last hit data.
    #[inline]
    fn save_last_hit_index(&self, path: &Path) -> Result<(), Error> {
        {
            let max = f64::from(*self.last_hit_index.max()? + 1) + 1.0e-3;
            let linear = self.last_hit_index.map(|x| f64::from(*x + 1) + 1.0e-3) / max;

            let img = linear.map(|x| self.scale.get(*x as f32));
            let p = path.join("last_hit.png");
            println!("Saving: {}", p.display());
            img.save(&p)?;
        }

        {
            let edges = img::find_edges(&self.last_hit_index, LAST_HIT_RAD)?;
            let max = edges.max()? + 1.0e-3;
            let linear = edges.map(|x| *x + 1.0e-3) / max;

            {
                let img = linear.map(|x| self.black_scale.get(*x as f32));
                let p = path.join("last_hit_edges_bk.png");
                println!("Saving: {}", p.display());
                img.save(&p)?;
            }

            {
                let img = linear.map(|x| self.outline_scale.get(*x as f32));
                let p = path.join("last_hit_edges_ol.png");
                println!("Saving: {}", p.display());
                img.save(&p)
            }
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.time += &rhs.time;
        self.first_hit_index += &rhs.first_hit_index;
        self.last_hit_index += &rhs.last_hit_index;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // Get current time string.
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let path = out_dir.join(time);
        std::fs::create_dir(&path)?;

        self.save_main_img(&path)?;
        self.save_time_img(&path)?;
        self.save_first_hit_index(&path)?;
        self.save_last_hit_index(&path)
    }
}
