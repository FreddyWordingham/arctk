//! Simulation input.

use crate::{
    clone,
    geom::Track,
    math::{Mat4, Vec3},
    ord::{Build, X, Y},
    sim::mcrt::Frame,
};
use arctk_attr::file;

/// Near clipping distance (m).
const NEAR_CLIP: f64 = 1.0e-3;

/// Far clipping distance (m).
const FAR_CLIP: f64 = 1.0e3;

/// Picture frame builder.
#[file]
pub struct FilmBuilder {
    /// Positioning.
    pos: Track,
    /// Targetting.
    tar: Track,
    /// Horizontal field-of-view (deg).
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Frames.
    frames: i32,
}

impl FilmBuilder {
    clone!(res, [usize; 2]);
}

impl Build for FilmBuilder {
    type Inst = Vec<Frame>;

    #[inline]
    fn build(self) -> Self::Inst {
        debug_assert!(self.frames > 0);

        let aspect_ratio = self.res[X] as f64 / self.res[Y] as f64;
        let fovy = self.fov.to_radians() / aspect_ratio;

        let mut frames = Vec::with_capacity(self.frames as usize);
        for n in 0..self.frames {
            let pos = self.pos.sample(n, self.frames);
            let tar = self.tar.sample(n, self.frames);

            let view = Mat4::look_at_rh(&pos, &tar, &Vec3::z_axis());
            let proj = Mat4::new_perspective(aspect_ratio, fovy, NEAR_CLIP, FAR_CLIP);

            frames.push(Frame::new(pos, view, proj, self.res));
        }

        frames
    }
}
