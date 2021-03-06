//! Hit side enumeration.

use crate::math::Dir3;

/// Side of a surface hit.
#[derive(Clone)]
pub enum Side {
    /// Inside of surface hit. d.dot(n) > 0.0
    Inside(Dir3),
    /// Outside of surface hit. d.dot(n) < 0.0
    Outside(Dir3),
}

impl Side {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(dir: &Dir3, norm: Dir3) -> Self {
        if dir.dot(&norm) < 0.0 {
            Self::Outside(norm)
        } else {
            Self::Inside(-norm)
        }
    }

    /// Check if the side is an inside.
    #[inline]
    #[must_use]
    pub const fn is_inside(&self) -> bool {
        match *self {
            Self::Inside(..) => true,
            Self::Outside(..) => false,
        }
    }

    /// Reference the surface-normal vector.
    /// This points away from the constructing direction normal.
    #[inline]
    #[must_use]
    pub const fn norm(&self) -> &Dir3 {
        match *self {
            Self::Inside(ref dir) | Self::Outside(ref dir) => dir,
        }
    }
}
