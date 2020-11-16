//! Event enumeration.

use crate::geom::Hit;

/// Event determination enumeration.
pub enum Event<'a, T> {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Surface hit.
    Surface(Hit<'a, T>),
}

impl<'a, T> Event<'a, T> {
    /// Construct a new instance.
    /// Surface interactions are prioritised, then boundary collisions, and finally scattering events.
    #[inline]
    #[must_use]
    pub fn new(voxel_dist: f64, surf_hit: Option<Hit<'a, T>>, bump_dist: f64) -> Self {
        debug_assert!(voxel_dist > 0.0);
        debug_assert!(bump_dist > 0.0);

        if let Some(hit) = surf_hit {
            if voxel_dist < (hit.dist() + bump_dist) {
                return Self::Voxel(voxel_dist);
            }
            return Self::Surface(hit);
        }

        Self::Voxel(voxel_dist)
    }
}
