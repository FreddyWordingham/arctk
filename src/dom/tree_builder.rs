//! Oct-tree construction settings.

use crate::dom::{Surface, Tree};

/// Tree construction settings.
pub struct TreeBuilder {
    /// Target maximum number of triangles per cell.
    pub tar_tris: usize,
    /// Maximum mesh depth.
    pub max_depth: u32,
    /// Collision detection expansion parameter.
    pub padding: f64,
}

impl TreeBuilder {
    /// Build a Tree instance.
    #[inline]
    #[must_use]
    pub fn build<'a, T>(&self, surfs: &'a [Surface<T>]) -> Tree<'a, T> {
        Tree::new(self, surfs)
    }
}
