//! Group layout structure.

use crate::{access, order, Error, Group, Grp, Load, Save};
use ndarray::Array3;
use num::traits::{One, Zero};
use std::path::Path;

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

    /// Construct a new instance from an actual group map.
    #[inline]
    #[must_use]
    pub fn new(gs: &Array3<Group>) -> Self {
        let mut groups = Vec::new();
        for g in gs {
            if !groups.contains(g) {
                groups.push(g.clone());
            }
        }
        groups.sort();

        let ids: Vec<_> = groups.iter().enumerate().collect();
        let map = gs.map(|g| {
            for (id, group) in &ids {
                if group == &g {
                    return *id;
                }
            }
            panic!("Missing group entry!");
        });

        Self { groups, map }
    }

    /// Construct a new instance.
    /// # Errors
    /// if the material map can not be loaded.
    #[allow(clippy::option_expect_used)]
    #[inline]
    pub fn load(path: &Path, groups: Vec<Group>) -> Result<Self, Error> {
        let map: Array3<i8> = Array3::load(path)?;
        let map = map.map(|x| *x as usize);
        let kinds = order::kinds(
            map.as_slice()
                .expect("Could not create slice from loaded group map."),
        );
        if kinds != groups.len() {
            panic!("Number of groups in the map does not match number in the given list.");
        }

        Ok(Self { groups, map })
    }

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

impl Save for Layout {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        for group in &self.groups {
            let p = path.join(format!("{}.nc", group));
            println!("Saving: {}", p.display());
            self.group_stencil::<i8>(group).save(&p)?;
        }

        let p = path.join(format!("{}.nc", "composite"));
        println!("Saving: {}", p.display());
        self.map.map(|x| (x + 1) as i8).save(&p)
    }
}
