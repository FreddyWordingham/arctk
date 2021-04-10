//! Simulation input.

use crate::{
    access,
    math::{Mat4, Pos3},
    ord::{X, Y},
};

/// Real-space to frame-space transformer.
pub struct Frame {
    /// Position.
    pos: Pos3,
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
    access!(pos, Pos3);
    access!(res, [usize; 2]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, model: Mat4, view: Mat4, proj: Mat4, res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            pos,
            model,
            view,
            proj,
            res,
        }
    }

    /// Transform a position in real-space to a pixel element.
    #[inline]
    #[must_use]
    pub fn transform(&self, pos: &Pos3) -> Option<[usize; 2]> {
        let p = self.model * self.view * self.proj * pos.to_homogeneous();

        if !(-1.0..1.0).contains(&p.x) || !(-1.0..1.0).contains(&p.y) {
            return None;
        }

        Some([
            ((p.x + 1.0) * 0.5 * self.res[X] as f64) as usize,
            ((p.y + 1.0) * 0.5 * self.res[Y] as f64) as usize,
        ])
    }
}
