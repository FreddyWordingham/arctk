//! Super sampling implementation.

use crate::{geom::Cube, math::Pos3, tools::linear_to_three_dim, tools::Valid};
use arctk_attr::load;
use rand::Rng;

/// Super sampling types.
#[load]
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

impl Valid for SuperSample {
    #[inline]
    fn check(&self) -> bool {
        match *self {
            Self::Off => true,
            Self::Uniform([nx, ny, nz]) => (nx > 0) && (ny > 0) && (nz > 0),
            Self::Random(n) => n > 0,
        }
    }
}
