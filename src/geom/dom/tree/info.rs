//! Information methods.

use crate::geom::Tree;

impl<'a, T> Tree<'a, T> {
    /// Determine the number of cells contained within the cell.
    /// This cell is included in the count.
    #[inline]
    #[must_use]
    pub fn num_cells(&self) -> usize {
        match *self {
            Self::Root { ref children, .. } | Self::Branch { ref children, .. } => {
                1 + children.iter().map(|ch| ch.num_cells()).sum::<usize>()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine the number leaf of cells contained within the cell.
    /// This cell is included in the count.
    #[inline]
    #[must_use]
    pub fn num_leaf_cells(&self) -> usize {
        match *self {
            Self::Root { ref children, .. } | Self::Branch { ref children, .. } => {
                children.iter().map(|ch| ch.num_leaf_cells()).sum::<usize>()
            }
            Self::Leaf { .. } => 1,
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the number of reference pairs contained within this cell and all child cells.
    #[inline]
    #[must_use]
    pub fn num_tri_refs(&self) -> usize {
        match *self {
            Self::Root { ref children, .. } | Self::Branch { ref children, .. } => {
                children.iter().map(|c| c.num_tri_refs()).sum()
            }
            Self::Leaf { ref tris, .. } => tris.len(),
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the maximum depth from this cell to a terminal cell.
    #[inline]
    #[must_use]
    pub fn depth(&self) -> usize {
        match *self {
            Self::Root { ref children, .. } => children.iter().map(|c| c.depth()).max().unwrap(),
            Self::Branch { ref children, .. } => {
                1 + children.iter().map(|c| c.depth()).max().unwrap()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }
}
