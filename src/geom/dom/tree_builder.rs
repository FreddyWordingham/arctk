//! Adaptive tree builder.

use crate::clone;
use arctk_attr::load;

/// Tree builder.
#[load]
pub struct TreeBuilder {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: i32,
    /// Collision detection padding.
    padding: f64,
}

impl TreeBuilder {
    clone!(tar_tris, usize);
    clone!(max_depth, i32);
    clone!(padding, f64);

    // /// Construct a new tree root cell.
    // /// Root cell has a depth of zero.
    // #[inline]
    // #[must_use]
    // pub fn build(self, surfs: &'a Set<Mesh>) -> Cell {
    // }
}
