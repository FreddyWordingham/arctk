//! Cartesian dimensions.

/// Cartesian coordinate system.
pub enum Cartesian {
    /// First spatial dimension.
    X,
    /// Second spatial dimension.
    Y,
    /// Third spatial dimension.
    Z,
}

/// Cartesian X convenience indexing constant.
pub const X: usize = Cartesian::X as usize;

/// Cartesian Y convenience indexing constant.
pub const Y: usize = Cartesian::Y as usize;

/// Cartesian Z convenience indexing constant.
pub const Z: usize = Cartesian::Z as usize;
