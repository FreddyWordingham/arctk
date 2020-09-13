//! Spherical-polar dimensions.

/// Spherical-polar coordinate system.
pub enum Spherical {
    /// Radial distance. [0 : inf]
    Rho,
    /// Angle. [0 : Pi]
    Theta,
    /// Azimuthal angle. [0 : 2*Pi]
    Phi,
}

/// Spherical-polar and plane-polar rho convenience indexing constant.
pub const RHO: usize = Spherical::Rho as usize;

/// Spherical-polar and plane-polar theta convenience indexing constant.
pub const THETA: usize = Spherical::Theta as usize;

/// Spherical-polar phi convenience indexing constant.
pub const PHI: usize = Spherical::Phi as usize;
