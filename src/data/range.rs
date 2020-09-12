//! Range implementation.

use crate::clone;
use arctk_attr::load;
use std::{
    f64::{INFINITY, NEG_INFINITY},
    fmt::{Display, Formatter, Result},
};

/// One-dimensional inclusive Range.
#[load]
#[derive(Clone, PartialEq)]
pub struct Range {
    /// Minimum bound.
    min: f64,
    /// Maximum bound.
    max: f64,
}

impl Range {
    clone!(min, f64);
    clone!(max, f64);

    /// Construct a new Range.
    #[inline]
    #[must_use]
    pub fn new(min: f64, max: f64) -> Self {
        debug_assert!(min < max);

        Self { min, max }
    }

    /// Construct an infinite Range.
    #[inline]
    #[must_use]
    pub const fn new_infinite() -> Self {
        Self {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }

    /// Calculate the width of the Range.
    #[inline]
    #[must_use]
    pub fn width(&self) -> f64 {
        self.max - self.min
    }

    /// Determine if a value is contained within the Range.
    #[inline]
    #[must_use]
    pub fn contains(&self, x: f64) -> bool {
        if x < self.min || x > self.max {
            return false;
        }

        true
    }

    /// Determine if the Range intersects with another given Range.
    #[inline]
    #[must_use]
    pub fn intersect(&self, other: &Self) -> bool {
        if self.max < other.min || other.max < self.min {
            return false;
        }

        true
    }

    /// From a range of overlapping values.
    #[inline]
    #[must_use]
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        if !self.intersect(other) {
            return None;
        }

        let min = self.min.max(other.min);
        let max = self.max.min(other.max);

        Some(Self::new(min, max))
    }
}

impl Display for Range {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "{} -> {}", self.min, self.max)
    }
}
