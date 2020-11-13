//! Event enumeration.

use crate::geom::Hit;
use crate::ord::Key;

/// Event determination enumeration.
pub enum Event<'a> {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Scattering event.
    Scattering(f64),
    /// Surface hit.
    Surface(Hit<&'a Key>),
}

impl<'a> Event<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        voxel_dist: f64,
        scat_dist: f64,
        surf_hit: Option<Hit<&'a Key>>,
        bump_dist: f64,
    ) -> Self {
        debug_assert!(voxel_dist > 0.0);
        debug_assert!(scat_dist > 0.0);
        debug_assert!(bump_dist > 0.0);

        if let Some(hit) = surf_hit {
            if voxel_dist < hit.dist() {
                if scat_dist < voxel_dist {
                    return Self::Scattering(scat_dist);
                }
                return Self::Voxel(voxel_dist);
            }

            if scat_dist < hit.dist() {
                return Self::Scattering(scat_dist);
            }
            return Self::Surface(hit);
        }

        if scat_dist < voxel_dist {
            return Self::Scattering(scat_dist);
        }
        Self::Voxel(voxel_dist)
    }
}
