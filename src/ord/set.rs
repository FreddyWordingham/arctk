//! Data set.

use crate::{
    err::Error,
    file::{from_json, Build, Load},
    ord::Setup,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{
        btree_map::{IntoIter, Values},
        BTreeMap,
    },
    ops::Index,
    path::Path,
};

/// Data map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T>(BTreeMap<String, T>);

impl<T> Set<T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<String, T>) -> Self {
        debug_assert!(!map.is_empty());

        Self(map)
    }

    /// Construct an instance from a vector of pairs.
    /// # Errors
    /// if a the list contains a duplicate entry.
    #[inline]
    pub fn from_vec(list: Vec<(String, T)>) -> Result<Self, Error> {
        let mut map = BTreeMap::new();

        for (key, item) in list {
            if map.contains_key(&key) {
                return Err(Error::Text(format!("Duplicate entries for group: {}", key)));
            }

            map.insert(key, item);
        }

        Ok(Self::new(map))
    }

    /// Get the number of items.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterate over the values.
    #[inline]
    #[must_use]
    pub fn values(&self) -> Values<String, T> {
        self.0.values()
    }
}

impl<T> Index<&str> for Set<T> {
    type Output = T;

    #[inline]
    fn index(&self, name: &str) -> &Self::Output {
        &self.0[name]
    }
}

impl<T> IntoIterator for Set<T> {
    type Item = (String, T);
    type IntoIter = IntoIter<String, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Load for Set<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
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

impl<'a, T, S: Setup<'a, T>> Setup<'a, T> for Set<S> {
    type Inst = Set<S::Inst>;

    #[inline]
    fn setup(self, set: &'a Set<T>) -> Result<Self::Inst, Error> {
        let mut list = Vec::with_capacity(self.0.len());
        for (name, val) in self.0 {
            list.push((name, val.setup(set)?));
        }
        let x = Set::from_vec(list);
        x
    }
}
