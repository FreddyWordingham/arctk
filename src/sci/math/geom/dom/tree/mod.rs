//! Adaptive tree cell scheme.

use crate::{Aabb, Grp, SmoothTriangle};

/// Tree cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell<'a> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
        /// Intersecting triangles.
        tris: Vec<(&'a Grp, &'a SmoothTriangle)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

impl<'a> Cell<'a> {
    /// Reference the cell's boundary.
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Aabb {
        match self {
            Self::Root { boundary, .. }
            | Self::Branch { boundary, .. }
            | Self::Leaf { boundary, .. }
            | Self::Empty { boundary, .. } => boundary,
        }
    }
}

pub mod construct;
pub mod display;
pub mod info;
pub mod observe;
pub mod scan;
pub mod search;
pub mod settings;

pub use self::{construct::*, display::*, info::*, observe::*, scan::*, search::*, settings::*};
