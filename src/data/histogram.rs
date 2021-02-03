//! Histogram implementation.

use crate::{
    access,
    err::Error,
    fs::Save,
    tools::{Binner, Range},
};
use ndarray::Array1;
use std::{fs::File, io::Write, ops::AddAssign, path::Path};

/// Static range, constant bin width, Histogram.
pub struct Histogram {
    /// Binner.
    binner: Binner,
    /// Count data.
    counts: Array1<f64>,
}

impl Histogram {
    access!(binner, Binner);
    access!(counts, Array1<f64>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(min: f64, max: f64, bins: u64) -> Self {
        debug_assert!(min < max);
        debug_assert!(bins > 0);

        Self {
            binner: Binner::new(Range::new(min, max), bins),
            counts: Array1::zeros(bins as usize),
        }
    }

    /// Construct a new instance using a range.
    #[inline]
    #[must_use]
    pub fn new_range(range: Range, bins: u64) -> Self {
        Self {
            binner: Binner::new(range, bins),
            counts: Array1::zeros(bins as usize),
        }
    }

    /// Increment the bin corresponding to x by unity.
    #[inline]
    pub fn collect(&mut self, x: f64) {
        debug_assert!(self.binner.range().contains(x));

        let index = self.binner.bin(x);
        self.counts[index] += 1.0;
    }

    /// Increment the bin corresponding to x by a given weight.
    #[inline]
    pub fn collect_weight(&mut self, x: f64, weight: f64) {
        debug_assert!(self.binner.range().contains(x));
        debug_assert!(weight > 0.0);

        let index = self.binner.bin(x);
        self.counts[index] += weight;
    }

    /// Increment the bin corresponding to x by unity if x is contained within the range.
    #[inline]
    pub fn try_collect(&mut self, x: f64) {
        if let Some(index) = self.binner.try_bin(x) {
            self.counts[index] += 1.0;
        }
    }

    /// Increment the bin corresponding to x by unity if x is contained within the range.
    #[inline]
    pub fn try_collect_weight(&mut self, x: f64, weight: f64) {
        if let Some(index) = self.binner.try_bin(x) {
            self.counts[index] += weight;
        }
    }
}

impl AddAssign<&Self> for Histogram {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!(self.binner == rhs.binner);
        debug_assert!(self.counts.len() == rhs.counts.len());

        self.counts += &rhs.counts;
    }
}

impl Save for Histogram {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
        let mut file = File::create(path)?;

        let mut center = self.binner.range().min();
        let delta = self.binner.range().width() / (self.counts.len() - 1) as f64;
        for count in &self.counts {
            center += delta;
            writeln!(file, "{:>32}, {:<32}", center, count)?;
        }

        Ok(())
    }
}

impl Display for Histogram {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Interface(in_mat, out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Spectrometer => {
                write!(fmt, "Spectrometer")
            }
        }
    }
}
