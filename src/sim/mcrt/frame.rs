//! Simulation input.

use crate::{
    math::{Mat4, Pos3},
    ord::{X, Y},
};

/// Real-space to frame-space transformer.
pub struct Frame {
    /// Model matrix.
    model: Mat4,
    /// View matrix.
    view: Mat4,
    /// Projection matrix.
    proj: Mat4,
    /// Image resolution.
    res: [usize; 2],
}

impl Frame {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(model: Mat4, view: Mat4, proj: Mat4, res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            model,
            view,
            proj,
            res,
        }
    }

    /// Transform a position in real-space to a pixel element.
    #[inline]
    #[must_use]
    pub fn transform(&self, pos: &Pos3) -> [usize; 2] {
        let p = self.model * self.view * self.proj * pos.to_homogeneous();

        [p.x as usize, p.y as usize]
    }
}
