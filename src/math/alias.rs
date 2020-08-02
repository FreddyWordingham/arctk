//! Aliases.

use nalgebra::{
    Point2, Point3, Point4, Rotation2, Rotation3, Similarity2, Similarity3, Unit, Vector2, Vector3,
    Vector4,
};

/// Two-dimensional vector alias.
pub type Vec2 = Vector2<f64>;
/// Three-dimensional vector alias.
pub type Vec3 = Vector3<f64>;
/// Four-dimensional vector alias.
pub type Vec4 = Vector4<f64>;

/// Normalised two-dimensional vector alias.
pub type Dir2 = Unit<Vector2<f64>>;
/// Normalised three-dimensional vector alias.
pub type Dir3 = Unit<Vector3<f64>>;
/// Normalised four-dimensional vector alias.
pub type Dir4 = Unit<Vector4<f64>>;

/// Two-dimensional position alias.
pub type Pos2 = Point2<f64>;
/// Three-dimensional position alias.
pub type Pos3 = Point3<f64>;
/// Four-dimensional position alias.
pub type Pos4 = Point4<f64>;

/// Two-dimensional rotation alias.
pub type Rot2 = Rotation2<f64>;
/// Three-dimensional rotation alias.
pub type Rot3 = Rotation3<f64>;

/// Two-dimensional transformation alias.
pub type Trans2 = Similarity2<f64>;
/// Three-dimensional transformation alias.
pub type Trans3 = Similarity3<f64>;
