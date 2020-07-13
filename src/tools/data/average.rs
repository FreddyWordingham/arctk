//! Rolling average implementation.

use crate::clone;
use std::ops::AddAssign;

/// Rolling average value recording.
#[derive(Clone, Default)]
pub struct Average {
    /// Current average value.
    total: f64,
    /// Total counts so far.
    counts: i32,
}

impl Average {
    clone!(total, f64);
    clone!(counts, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(total: f64, counts: i32) -> Self {
        debug_assert!(counts >= 0);

        Self { total, counts }
    }

    /// Calculate the average value.
    #[inline]
    #[must_use]
    pub fn ave(&self) -> f64 {
        if self.counts > 0 {
            self.total / f64::from(self.counts)
        } else {
            0.0
        }
    }
}

impl AddAssign<Self> for Average {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.total += rhs.total;
        self.counts += rhs.counts;
    }
}

impl AddAssign<&Self> for Average {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.total += rhs.total;
        self.counts += rhs.counts;
    }
}

impl AddAssign<f64> for Average {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.total += rhs;
        self.counts += 1;
    }
}
