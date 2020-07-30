//! Material map.

use crate::{access, order, Error, Group, Grp, Load};
use ndarray::Array3;
use std::path::Path;

/// Group grid map.
pub struct GroupMap {
    /// List of entries.
    groups: Vec<Group>,
    /// Id (index) map.
    map: Array3<i32>,
}

impl GroupMap {
    access!(groups, Vec<Group>);
    access!(map, Array3<i32>);

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
                    return *id as i32;
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
        let map = Array3::load(path)?;
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
    fn id_to_group(&self, id: i32) -> &Grp {
        &self.groups[id as usize]
    }

    /// Convert a valid group to it's corresponding id.
    #[inline]
    #[must_use]
    fn group_to_id(&self, group: &Group) -> i32 {
        self.groups
            .binary_search(group)
            .expect("Invalid group entry.") as i32
    }

    /// Get the group at a given index of the map.
    #[inline]
    #[must_use]
    pub fn group_at_index(&self, index: [usize; 3]) -> &Grp {
        &self.groups[self.map[index] as usize]
    }

    /// Get the map for a given group.
    #[inline]
    #[must_use]
    pub fn group_map(&self, group: &Group) -> Array3<f64> {
        let
    }
}
