//! Access macro.

/// Succinctly create a reference method for the given variable.
#[macro_export]
macro_rules! access {
    ($field:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> &$type {
            &self.$field
        }
    };

    ($field:ident, $setter:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> &$type {
            &self.$field
        }

        #[allow(clippy::mut_mut)]
        #[inline]
        #[must_use]
        pub fn $setter(&mut self) -> &mut $type {
            &mut self.$field
        }
    };
}

/// Succinctly create a clone method for the given variable.
#[macro_export]
macro_rules! clone {
    ($field:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> $type {
            self.$field
        }
    };

    ($field:ident, $setter:ident, $type:ty) => {
        #[inline]
        #[must_use]
        pub const fn $field(&self) -> $type {
            self.$field
        }

        #[inline]
        #[must_use]
        pub fn $setter(&mut self) -> &mut $type {
            &mut self.$field
        }
    };
}
