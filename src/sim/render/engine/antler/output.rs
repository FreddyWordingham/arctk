//! Output structure.

use crate::{render::img, Dir3, Error, Image, Save, X, Y, Z};
use ndarray::{Array2, Zip};
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
    /// Dimension colour scales.
    dim_scales: [Gradient<LinSrgba>; 3],
    /// Base image.
    pub image: Image,
    /// Time data.
    pub time: Array2<f64>,
    /// First hit index.
    pub first_hit_index: Array2<i32>,
    /// First hit normal.
    pub first_hit_norm: Array2<Option<Dir3>>,
    /// Las hit index.
    pub last_hit_index: Array2<i32>,
    /// Last hit normal.
    pub last_hit_norm: Array2<Option<Dir3>>,
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
        dim_scales: [Gradient<LinSrgba>; 3],
    ) -> Self {
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            scale,
            black_scale,
            outline_scale,
            dim_scales,
            image: Image::default(img_res),
            time: Array2::zeros(img_res),
            first_hit_index: Array2::zeros(img_res),
            first_hit_norm: Array2::default(img_res),
            last_hit_index: Array2::zeros(img_res),
            last_hit_norm: Array2::default(img_res),
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

    /// Save the first hit normal data.
    #[inline]
    fn save_first_hit_norm(&self, path: &Path) -> Result<(), Error> {
        let img = self.first_hit_norm.map(|norm| {
            if let Some(n) = norm {
                self.dim_scales[X].get(n.x.abs() as f32)
                    + self.dim_scales[Y].get(n.y.abs() as f32)
                    + self.dim_scales[Z].get(n.z.abs() as f32)
            } else {
                LinSrgba::new(0.0, 0.0, 0.0, 1.0)
            }
        });
        let p = path.join("first_hit_norm.png");
        println!("Saving: {}", p.display());
        img.save(&p)
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

    /// Save the last hit normal data.
    #[inline]
    fn save_last_hit_norm(&self, path: &Path) -> Result<(), Error> {
        let img = self.last_hit_norm.map(|norm| {
            if let Some(n) = norm {
                self.dim_scales[X].get(n.x.abs() as f32)
                    + self.dim_scales[Y].get(n.y.abs() as f32)
                    + self.dim_scales[Z].get(n.z.abs() as f32)
            } else {
                LinSrgba::new(1.0, 1.0, 1.0, 1.0)
            }
        });
        let p = path.join("last_hit_norm.png");
        println!("Saving: {}", p.display());
        img.save(&p)
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.time += &rhs.time;
        self.first_hit_index += &rhs.first_hit_index;
        self.last_hit_index += &rhs.last_hit_index;

        Zip::from(&mut self.first_hit_norm)
            .and(&rhs.first_hit_norm)
            .apply(|a, &b| *a = if a.is_some() { *a } else { b });
        Zip::from(&mut self.last_hit_norm)
            .and(&rhs.last_hit_norm)
            .apply(|a, &b| *a = if a.is_some() { *a } else { b });
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
        self.save_first_hit_norm(&path)?;
        self.save_last_hit_index(&path)?;
        self.save_last_hit_norm(&path)
    }
}
