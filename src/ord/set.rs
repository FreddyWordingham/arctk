//! Data set.

use crate::{
    err::Error,
    file::{from_json, Build, Load},
    ord::Register,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

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

    /// Construct a register and store the data in a vector.
    #[inline]
    #[must_use]
    pub fn reg(self) -> (Vec<T>, Register) {
        let mut names = Vec::with_capacity(self.0.len());
        let mut values = Vec::with_capacity(self.0.len());

        for (name, value) in self.0 {
            names.push(name);
            values.push(value);
        }

        (values, Register::new(names))
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
