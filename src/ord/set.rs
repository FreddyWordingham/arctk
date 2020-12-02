//! Data set.

use crate::{
    err::Error,
    file::{from_json, Build, Load},
    ord::{Link, Name},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{
        btree_map::{IntoIter, Values},
        BTreeMap,
    },
    path::Path,
};

/// Data map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T>(BTreeMap<Name, T>);

impl<T> Set<T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Name, T>) -> Self {
        debug_assert!(!map.is_empty());

        Self(map)
    }

    /// Construct an instance from a vector of pairs.
    /// # Errors
    /// if a the list contains a duplicate entry.
    #[inline]
    pub fn from_vec(list: Vec<(Name, T)>) -> Result<Self, Error> {
        let mut map = BTreeMap::new();

        for (key, item) in list {
            if map.contains_key(&key) {
                return Err(Error::Text(format!("Duplicate entries for group: {}", key)));
            }

            map.insert(key, item);
        }

        Ok(Self::new(map))
    }

    /// Find if the number of entries is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the number of entries.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterate over the values.
    #[inline]
    #[must_use]
    pub fn values(&self) -> Values<Name, T> {
        self.0.values()
    }

    /// Get a value from the map.
    #[inline]
    #[must_use]
    pub fn get(&self, name: &Name) -> Option<&T> {
        self.0.get(name)
    }
}

impl<T> IntoIterator for Set<T> {
    type Item = (Name, T);
    type IntoIter = IntoIter<Name, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Load for Set<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load_data(path: &Path) -> Result<Self, Error> {
        from_json(path)
    }
}

#[allow(clippy::use_self)]
impl<T: Build> Build for Set<T> {
    type Inst = Set<T::Inst>;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut map = BTreeMap::new();

        for (key, item) in self.0 {
            map.insert(key, item.build(in_dir)?);
        }

        Ok(Self::Inst::new(map))
    }
}

#[allow(clippy::use_self)]
impl<'a, T, S: Link<'a, T>> Link<'a, T> for Set<S> {
    type Inst = Set<S::Inst>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        self.0
            .values()
            .map(|v| v.requires())
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect()
    }

    #[inline]
    fn link(self, set: &'a Set<T>) -> Result<Self::Inst, Error> {
        let mut list = Vec::with_capacity(self.0.len());
        for (name, val) in self.0 {
            list.push((name, val.link(set)?));
        }
        Self::Inst::from_vec(list)
    }
}
