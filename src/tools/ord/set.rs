//! Set implementation.

use crate::{as_json, from_json, report, Build, Error, Group, Load, Save};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::Path,
};

/// Set alias.
type Map<T> = BTreeMap<Group, T>;

/// Set map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T>(Map<T>);

impl<T> Set<T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: Map<T>) -> Self {
        debug_assert!(!map.is_empty());

        Self(map)
    }

    /// Construct an instance from a vector.
    #[inline]
    #[must_use]
    pub fn from_vec(list: Vec<(Group, T)>) -> Self {
        let mut map = Map::new();

        for (group, item) in list {
            if map.contains_key(&group) {
                panic!("Duplicate entries for group: {}", group);
            }

            map.insert(group, item);
        }

        Self::new(map)
    }

    /// Access the stored map.
    #[inline]
    #[must_use]
    pub const fn map(&self) -> &Map<T> {
        &self.0
    }

    /// Access the stored map mutably.
    #[inline]
    #[must_use]
    pub fn mut_map(&mut self) -> &mut Map<T> {
        &mut self.0
    }

    /// Determine the index of a given key within the Set.
    /// Return None if the key does not exist.
    #[inline]
    #[must_use]
    pub fn index_of(&self, k: &str) -> Option<usize> {
        for (index, key) in self.0.keys().enumerate() {
            if k == key {
                return Some(index);
            }
        }

        None
    }
}

impl<T> Load for Set<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        println!("loading: {}", path.display());
        from_json(path)
    }
}

impl<T: Serialize> Save for Set<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        as_json(self, path)
    }
}

impl<T: Build> Build for Set<T> {
    type Inst = Set<T::Inst>;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut map = Map::new();

        for (group, item) in self.0 {
            if map.contains_key(&group) {
                panic!("Duplicate entries for group: {}", group);
            }

            map.insert(group, item.build(in_dir)?);
        }

        Ok(Self::Inst::new(map))
    }
}

impl<T: Display> Display for Set<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let mut items = self.0.iter();

        if let Some((group, item)) = items.next() {
            write!(
                fmt,
                "{}",
                report::obj(&format!("[{}]", group), item).expect("Could not format field.")
            )?;
        }

        for (group, item) in items {
            write!(
                fmt,
                "\n{}",
                report::obj(&format!("[{}]", group), item).expect("Could not format field.")
            )?;
        }

        Ok(())
    }
}
