//! Hit side.

use nalgebra::{Unit, Vector3};

/// Differentiates the side of a surface.
#[allow(clippy::exhaustive_enums)]
#[derive(Clone)]
pub enum Side {
    /// Inside surface.
    Inside(Unit<Vector3<f64>>),
    /// Outside surface.
    Outside(Unit<Vector3<f64>>),
}

impl Side {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(dir: &Unit<Vector3<f64>>, norm: Unit<Vector3<f64>>) -> Self {
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
    pub const fn norm(&self) -> &Unit<Vector3<f64>> {
        match *self {
            Self::Inside(ref norm) | Self::Outside(ref norm) => norm,
        }
    }
}
