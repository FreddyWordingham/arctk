//! Rendering order enumeration implementation.

use attr::load;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::{Display, Formatter, Result};

/// Rendering order.
#[load]
#[derive(Clone)]
pub enum Order {
    /// Forward.
    Forward,
    /// Backward.
    Backward,
    /// Shuffle.
    Shuffle,
}

impl Order {
    /// Generate the order list.
    #[inline]
    #[must_use]
    pub fn list(&self, n: u64) -> Vec<u64> {
        match self {
            Self::Forward => (0..n).collect(),
            Self::Backward => {
                let mut ord: Vec<u64> = (0..n).collect();
                ord.reverse();
                ord
            }
            Self::Shuffle => {
                let mut ord: Vec<u64> = (0..n).collect();
                ord.shuffle(&mut thread_rng());
                ord
            }
        }
    }
}

impl Display for Order {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Forward => "Forward".to_string(),
            Self::Backward => "Backward".to_string(),
            Self::Shuffle => "Shuffle".to_string(),
        };
        write!(fmt, "{}", kind)
    }
}
