//! Adaptive tree cell scheme.

use crate::geom::{Cube, SmoothTriangle};

/// Tree cell enumeration.
pub enum Tree<'a, T> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Cube,
        /// Children.
        children: [Box<Tree<'a, T>>; 8],
    },
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Cube,
        /// Children.
        children: [Box<Tree<'a, T>>; 8],
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Cube,
        /// Intersecting triangles.
        tris: Vec<(T, &'a SmoothTriangle)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Cube,
    },
}

impl<'a, T> Tree<'a, T> {
    /// Reference the cell's boundary.
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Cube {
        match self {
            Self::Root { boundary, .. }
            | Self::Branch { boundary, .. }
            | Self::Leaf { boundary, .. }
            | Self::Empty { boundary, .. } => boundary,
        }
    }
}

// pub mod display;
// pub mod info;
// pub mod observe;
// pub mod scan;
// pub mod search;
// pub mod settings;

// pub use self::{display::*, info::*, observe::*, scan::*, search::*, settings::*};
