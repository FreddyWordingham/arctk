//! Peel-off function.

use crate::{math::Pos3, sim::mcrt::Photon};

/// Calculate the change in weight over a give flight towards a given point.
/// Perform a peel off event.
#[inline]
#[must_use]
pub const fn peel_off(mut _phot: Photon, _pos: Pos3) -> Option<f64> {
    None
}
