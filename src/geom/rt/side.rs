//! Hit side enumeration.

use crate::math::Dir3;

/// Side of a surface hit.
#[derive(Clone)]
pub enum Side {
    /// Inside of surface hit. d.dot(n) > 0.0
    Inside {
        /// Facing surface normal vector.
        norm: Dir3,
    },
    /// Outside of surface hit. d.dot(n) < 0.0
    Outside {
        /// Facing surface normal vector.
        norm: Dir3,
    },
}

impl Side {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(dir: &Dir3, norm: Dir3) -> Self {
        if dir.dot(&norm) < 0.0 {
            Self::Outside { norm }
        } else {
            Self::Inside { norm: -norm }
        }
    }

    /// Reference the normal vector.
    #[inline]
    #[must_use]
    pub const fn norm(&self) -> &Dir3 {
        match *self {
            Self::Inside { ref norm } | Self::Outside { ref norm } => norm,
        }
    }

    /// Reverse the side.
    #[inline]
    #[must_use]
    pub const fn flip(self) -> Self {
        match self {
            Self::Inside { norm } => Self::Outside { norm },
            Self::Outside { norm } => Self::Inside { norm },
        }
    }

    /// Check if the side is an inside.
    #[inline]
    #[must_use]
    pub const fn is_inside(&self) -> bool {
        match *self {
            Self::Inside { .. } => true,
            Self::Outside { .. } => false,
        }
    }

    /// Check if the side is an outside.
    #[inline]
    #[must_use]
    pub const fn is_outside(&self) -> bool {
        match *self {
            Self::Inside { .. } => false,
            Self::Outside { .. } => true,
        }
    }
}
