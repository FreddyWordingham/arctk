//! Data set.

use crate::{
    err::Error,
    fmt_report,
    fs::{from_json, File, Load},
    ord::{Build, Link, Map, Name},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::{
    collections::btree_map::{IntoIter, Values},
    path::Path,
};

/// Data map.
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T>(Map<Name, T>);

impl<T> Set<T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: Map<Name, T>) -> Self {
        debug_assert!(!map.is_empty());

        Self(map)
    }

    /// Construct an instance from a vector of pairs.
    /// # Errors
    /// if a the list contains a duplicate entry.
    #[inline]
    pub fn from_pairs(list: Vec<(Name, T)>) -> Result<Self, Error> {
        let mut map = Map::new();

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

    /// Get a list of the names.
    #[inline]
    #[must_use]
    pub fn names_list(&self) -> Vec<Name> {
        self.0.keys().cloned().collect()
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

    /// Reference the internal map.
    #[inline]
    #[must_use]
    pub const fn map(&self) -> &Map<Name, T> {
        &self.0
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

impl<T> File for Set<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        from_json(path)
    }
}

#[allow(clippy::use_self)]
impl<T: Load> Load for Set<T> {
    type Inst = Set<T::Inst>;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut map = Map::new();

        for (key, item) in self.0 {
            map.insert(key, item.load(in_dir)?);
        }

        Ok(Self::Inst::new(map))
    }
}

#[allow(clippy::use_self)]
impl<T: Build> Build for Set<T> {
    type Inst = Set<T::Inst>;

    #[allow(clippy::expect_used)]
    #[must_use]
    #[inline]
    fn build(self) -> Self::Inst {
        let mut list = Vec::with_capacity(self.0.len());
        for (name, val) in self.0 {
            list.push((name, val.build()));
        }
        Self::Inst::from_pairs(list).expect("Failed to build set.")
    }
}

#[allow(clippy::use_self)]
impl<'a, T, S: Link<'a, T>> Link<'a, T> for Set<S> {
    type Inst = Set<S::Inst>;

    #[must_use]
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
        Self::Inst::from_pairs(list)
    }
}

impl<T: Display> Display for Set<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        for (key, val) in &self.0 {
            fmt_report!(fmt, val, &format!("{} ->", key));
        }
        Ok(())
    }
}
