//! Setup trait.

use crate::{
    err::Error,
    ord::{Name, Set},
};

/// Types implementing this trait can be linked to a set to produce a referenced type.
pub trait Link<'a, T> {
    /// Type to be built.
    type Inst;

    /// Get a list of all required resource keys.
    fn requires(&self) -> Vec<Name>;

    /// Link the instance type.
    /// # Errors
    /// if a field could not be referenced.
    fn link(self, set: &'a Set<T>) -> Result<Self::Inst, Error>;
}

#[allow(clippy::use_self)]
impl<'a, T, S: Link<'a, T>> Link<'a, T> for Vec<S> {
    type Inst = Vec<S::Inst>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        self.iter()
            .map(|v| v.requires())
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect()
    }

    #[inline]
    fn link(self, set: &'a Set<T>) -> Result<Self::Inst, Error> {
        let mut list = Vec::with_capacity(self.len());

        for x in self {
            list.push(x.link(set)?);
        }

        Ok(list)
    }
}
