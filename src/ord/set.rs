//! Data set.

use crate::{err::Error, ord::Register};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
    #[inline]
    #[must_use]
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
    pub fn reg(self) -> (Register, Vec<T>) {
        let mut names = Vec::with_capacity(self.0.len());
        let mut values = Vec::with_capacity(self.0.len());

        for (name, value) in self.0 {
            names.push(name);
            values.push(value);
        }

        (Register::new(names), values)
    }
}
