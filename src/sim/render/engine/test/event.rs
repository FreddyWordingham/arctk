//! Collision event enumeration.

use crate::Hit;

/// Event determination enumeration.
pub enum Event<'a> {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Surface hit.
    Surface(Hit<'a>),
}

impl<'a> Event<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(voxel_dist: f64, surf_hit: Option<Hit<'a>>) -> Self {
        debug_assert!(voxel_dist > 0.0);
        debug_assert!(surf_hit.is_none() || surf_hit.clone().unwrap().dist() > 0.0);

        if let Some(surf_hit) = surf_hit {
            if surf_hit.dist() <= voxel_dist {
                return Self::Surface(surf_hit);
            }
        }

        Self::Voxel(voxel_dist)
    }
}
