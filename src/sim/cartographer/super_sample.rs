//! Super sampling options.

use crate::{geom::Cube, math::Pos3, tools::linear_to_three_dim};
use arctk_attr::file;
use rand::Rng;
use std::fmt::{Display, Error, Formatter};

/// Super sampling types.
#[file]
#[derive(Clone)]
pub enum SuperSample {
    /// Single sample.
    Off,
    /// Uniform.
    Uniform([usize; 3]),
    /// Random.
    Random(usize),
}

impl SuperSample {
    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub const fn num_samples(&self) -> usize {
        match *self {
            Self::Off => 1,
            Self::Uniform([nx, ny, nz]) => nx * ny * nz,
            Self::Random(n) => n,
        }
    }

    /// Determine the nth sampling position.
    #[inline]
    #[must_use]
    pub fn sample<R: Rng>(&self, cube: &Cube, n: usize, rng: &mut R) -> Pos3 {
        match *self {
            Self::Off => cube.centre(),
            Self::Uniform(ref res) => cube.uniform_pos(res, &linear_to_three_dim(n, res)),
            Self::Random(..) => cube.rand_pos(rng),
        }
    }
}

impl Display for SuperSample {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Off => write!(fmt, "Off"),
            Self::Uniform([nx, ny, nz]) => write!(fmt, "[{} x {} x {}]", nx, ny, nz),
            Self::Random(n) => write!(fmt, "Random ({})", n),
        }
    }
}
