//! Event enumeration.

use crate::geom::Hit;

/// Event determination enumeration.
pub enum Event<'a, T> {
    /// Grid boundary collision.
    Grid(f64),
    /// Surface hit.
    Surface(Hit<'a, T>),
}

impl<'a, T> Event<'a, T> {
    /// Construct a new instance.
    /// Surface interactions are prioritised, then boundary collisions, and finally scattering events.
    #[inline]
    #[must_use]
    pub fn new(grid_dist: f64, surf_hit: Option<Hit<'a, T>>, bump_dist: f64) -> Self {
        debug_assert!(grid_dist > 0.0);
        debug_assert!(bump_dist > 0.0);

        if let Some(hit) = surf_hit {
            if grid_dist < (hit.dist() + bump_dist) {
                return Self::Grid(grid_dist);
            }
            return Self::Surface(hit);
        }

        Self::Grid(grid_dist)
    }
}
