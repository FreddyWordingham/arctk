//! Event enumeration.

use crate::{geom::Hit, ord::Key};

/// Event determination enumeration.
pub enum ClearEvent<'a> {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Surface hit.
    Surface(Hit<&'a Key>),
}

impl<'a> ClearEvent<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(voxel_dist: f64, surf_hit: Option<Hit<&'a Key>>, bump_dist: f64) -> Self {
        debug_assert!(voxel_dist > 0.0);
        debug_assert!(bump_dist > 0.0);

        if let Some(hit) = surf_hit {
            if voxel_dist < hit.dist() {
                return Self::Voxel(voxel_dist);
            }

            return Self::Surface(hit);
        }

        Self::Voxel(voxel_dist)
    }
}
