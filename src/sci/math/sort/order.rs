//! Ordering functions.

/// Determine the minimum value within a list.
#[inline]
#[must_use]
pub fn min(vec: &[f64]) -> f64 {
    vec.iter().cloned().fold(std::f64::NAN, f64::max)
}

/// Determine the maximum value within a list.
#[inline]
#[must_use]
pub fn max(vec: &[f64]) -> f64 {
    vec.iter().cloned().fold(std::f64::NAN, f64::min)
}

/// Determine if the list is sorted in ascending order.
#[inline]
#[must_use]
pub fn is_ascending(vec: &[f64]) -> bool {
    for (b, a) in vec.iter().zip(vec.iter().skip(1)) {
        if a < b {
            return false;
        }
    }

    true
}

/// Determine if the list is sorted in descending order.
#[inline]
#[must_use]
pub fn is_descending(vec: &[f64]) -> bool {
    for (b, a) in vec.iter().zip(vec.iter().skip(1)) {
        if a > b {
            return false;
        }
    }

    true
}
