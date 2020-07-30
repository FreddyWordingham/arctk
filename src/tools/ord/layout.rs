//! Group layout structure.

use crate::{access, Group, Grp};
use ndarray::Array3;
// use std::path::Path;
use num::traits::{One, Zero};

/// Group three-dimensional layout organiser.
pub struct Layout {
    /// List of entries.
    groups: Vec<Group>,
    /// Id (index) map.
    map: Array3<usize>,
}

impl Layout {
    access!(groups, Vec<Group>);
    access!(map, Array3<usize>);

    /// Convert a valid id to it's corresponding group.
    #[inline]
    #[must_use]
    fn id_to_group(&self, id: usize) -> &Grp {
        &self.groups[id]
    }

    /// Convert a valid group to it's corresponding id.
    #[inline]
    #[must_use]
    fn group_to_id(&self, group: &Group) -> usize {
        self.groups
            .binary_search(group)
            .expect("Invalid group entry.")
    }

    /// Get the group at a given index of the map.
    #[inline]
    #[must_use]
    pub fn group_at_index(&self, index: [usize; 3]) -> &Grp {
        self.id_to_group(self.map[index])
    }

    /// Create a stencil map for the given group.
    #[inline]
    #[must_use]
    pub fn group_stencil<T: Zero + One>(&self, group: &Group) -> Array3<T> {
        let index = self.group_to_id(group);
        self.map.map(|&x| {
            if x == index {
                num::one::<T>()
            } else {
                num::zero::<T>()
            }
        })
    }
}
