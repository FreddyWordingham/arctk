//! Event enumeration.

use crate::{report, Hit};
use std::fmt::{Display, Formatter, Result};

/// Event determination enumeration.
pub enum Event<'a> {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Scattering event.
    Scattering(f64),
    /// Surface hit.
    Surface(Hit<'a>),
}

impl<'a> Event<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(voxel_dist: f64, scat_dist: f64, surf_hit: Option<Hit<'a>>, bump_dist: f64) -> Self {
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

impl<'a> Display for Event<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let event = match self {
            Self::Voxel(dist) => format!("Voxel boundary {} [m]", dist),
            Self::Scattering(dist) => format!("Scatter {} [m]", dist),
            Self::Surface(hit) => format!("Surface {} [m]", hit.dist()),
        };

        write!(
            fmt,
            "{}",
            report::obj("event", event).expect("Could not format field.")
        )
    }
}
