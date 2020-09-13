//! Set implementation.

use crate::{
    err::Error,
    file::{as_json, from_json, Build, Load, Save},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, path::Path};

/// Set alias.
type Map<T, S> = BTreeMap<T, S>;

/// Set map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T: Ord, S>(Map<T, S>);

impl<T: Display + Ord + PartialEq, S> Set<T, S> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: Map<T, S>) -> Self {
        debug_assert!(!map.is_empty());

        Self(map)
    }

    /// Construct an instance from a vector.
    #[inline]
    #[must_use]
    pub fn from_vec(list: Vec<(T, S)>) -> Self {
        let mut map = Map::new();

        for (key, item) in list {
            if map.contains_key(&key) {
                panic!("Duplicate entries for group: {}", key);
            }

            map.insert(key, item);
        }

        Self::new(map)
    }

    /// Access the stored map.
    #[inline]
    #[must_use]
    pub fn map(&self) -> &Map<T, S> {
        &self.0
    }

    /// Access the stored map mutably.
    #[inline]
    #[must_use]
    pub fn mut_map(&mut self) -> &mut Map<T, S> {
        &mut self.0
    }

    /// Determine the index of a given key within the Set.
    /// Return None if the key does not exist.
    #[inline]
    #[must_use]
    pub fn index_of(&self, k: T) -> Option<usize> {
        for (index, key) in self.0.keys().enumerate() {
            if k == *key {
                return Some(index);
            }
        }

        None
    }
}

impl<T, S> Load for Set<T, S>
where
    for<'de> T: Deserialize<'de> + Ord,
    for<'de> S: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        println!("loading: {}", path.display());
        from_json(path)
    }
}

impl<T: Ord + Serialize, S: Serialize> Save for Set<T, S> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        as_json(self, path)
    }
}

impl<T: Display + Ord, S: Build> Build for Set<T, S> {
    type Inst = Set<T, S::Inst>;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut map = Map::new();

        for (key, item) in self.0 {
            if map.contains_key(&key) {
                panic!("Duplicate entries for key: {}", key);
            }

            map.insert(key, item.build(in_dir)?);
        }

        Ok(Self::Inst::new(map))
    }
}
