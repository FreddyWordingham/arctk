//! Spherical implementation.

/// Spherical-polar coordinate system.
pub enum Spherical {
    /// Radial distance.
    Rho,
    /// Polar angle.
    Theta,
    /// Azimuthal angle.
    Phi,
}

/// Spherical-polar rho convenience constant.
pub const RHO: usize = Spherical::Rho as usize;

/// Spherical-polar theta convenience constant.
pub const THETA: usize = Spherical::Theta as usize;

/// Spherical-polar phi convenience constant.
pub const PHI: usize = Spherical::Phi as usize;
