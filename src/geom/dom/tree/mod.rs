//! Adaptive tree cell scheme.

use crate::geom::{Cube, SmoothTriangle};

/// Tree cell enumeration.
pub enum Tree<'a> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Cube,
        /// Children.
        children: Box<[Tree<'a>; 8]>,
    },
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Cube,
        /// Children.
        children: Box<[Tree<'a>; 8]>,
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Cube,
        /// Intersecting triangles.
        tris: Vec<(&'a SmoothTriangle, usize)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Cube,
    },
}

impl<'a> Tree<'a> {
    /// Reference the cell's boundary.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Cube {
        match *self {
            Self::Root { ref boundary, .. }
            | Self::Branch { ref boundary, .. }
            | Self::Leaf { ref boundary, .. }
            | Self::Empty { ref boundary, .. } => boundary,
        }
    }
}

pub mod info;
pub mod observe;
pub mod scan;
pub mod search;

pub use self::{info::*, observe::*, scan::*, search::*};
