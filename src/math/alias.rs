//! Aliases.

use nalgebra::{
    Matrix2, Matrix3, Matrix4, Point2, Point3, Point4, Rotation2, Rotation3, Similarity2,
    Similarity3, Unit, Vector2, Vector3, Vector4,
};

/// Two-dimensional real-number vector alias.
pub type Vec2 = Vector2<f64>;
/// Three-dimensional real-number vector alias.
pub type Vec3 = Vector3<f64>;
/// Four-dimensional real-number vector alias.
pub type Vec4 = Vector4<f64>;

/// Two-by-two dimensional real-number matrix alias.
pub type Mat2 = Matrix2<f64>;
/// Three-by-three dimensional real-number matrix alias.
pub type Mat3 = Matrix3<f64>;
/// Four-by-four dimensional real-number matrix alias.
pub type Mat4 = Matrix4<f64>;

/// Two-dimensional discrete-number vector alias.
pub type Vec2I = Vector2<i32>;
/// Three-dimensional discrete-number vector alias.
pub type Vec3I = Vector3<i32>;
/// Four-dimensional discrete-number vector alias.
pub type Vec4I = Vector4<i32>;

/// Two-by-two dimensional discrete-number matrix alias.
pub type Mat2I = Matrix2<i32>;
/// Three-by-three dimensional discrete-number matrix alias.
pub type Mat3I = Matrix3<i32>;
/// Four-by-four dimensional discrete-number matrix alias.
pub type Mat4I = Matrix4<i32>;

/// Normalised two-dimensional vector alias.
pub type Dir2 = Unit<Vector2<f64>>;
/// Normalised three-dimensional vector alias.
pub type Dir3 = Unit<Vector3<f64>>;
/// Normalised four-dimensional vector alias.
pub type Dir4 = Unit<Vector4<f64>>;

/// Two-dimensional real-number position alias.
pub type Pos2 = Point2<f64>;
/// Three-dimensional real-number position alias.
pub type Pos3 = Point3<f64>;
/// Four-dimensional real-number position alias.
pub type Pos4 = Point4<f64>;

/// Two-dimensional discrete-number position alias.
pub type Pos2I = Point2<i32>;
/// Three-dimensional discrete-number position alias.
pub type Pos3I = Point3<i32>;
/// Four-dimensional discrete-number position alias.
pub type Pos4I = Point4<i32>;

/// Two-dimensional rotation alias.
pub type Rot2 = Rotation2<f64>;
/// Three-dimensional rotation alias.
pub type Rot3 = Rotation3<f64>;

/// Two-dimensional transformation alias.
pub type Trans2 = Similarity2<f64>;
/// Three-dimensional transformation alias.
pub type Trans3 = Similarity3<f64>;
