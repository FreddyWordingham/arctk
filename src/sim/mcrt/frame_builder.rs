//! Simulation input.

use crate::{
    math::{Mat4, Pos3, Vec3},
    ord::{Build, X, Y},
    sim::mcrt::Frame,
};
use arctk_attr::file;

/// Near clipping distance (m).
const NEAR_CLIP: f64 = 0.1;

/// Far clipping distance (m).
const FAR_CLIP: f64 = 100.0;

/// Picture frame builder.
#[file]
pub struct FrameBuilder {
    /// Position.
    pos: Pos3,
    /// Target.
    tar: Pos3,
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
}

impl Build for FrameBuilder {
    type Inst = Frame;

    #[inline]
    fn build(self) -> Self::Inst {
        let model = Mat4::identity();
        let view = Mat4::look_at_rh(&self.pos, &self.tar, &Vec3::z_axis());
        let aspect_ratio = self.res[X] as f64 / self.res[Y] as f64;
        let fovy = self.fov.to_radians() / aspect_ratio;
        let proj = Mat4::new_perspective(aspect_ratio, fovy, NEAR_CLIP, FAR_CLIP);

        Self::Inst::new(self.pos, model, view, proj, self.res)
    }
}
