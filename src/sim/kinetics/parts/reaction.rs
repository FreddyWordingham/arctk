//! Chemical reaction implementation.

use crate::{
    access,
    kinetics::{Chem, Rate},
};

/// Chemical reaction.
pub struct Reaction {
    /// Reaction reactants and their respective stoichiometric coefficients.
    reactants: Vec<(i32, Chem)>,
    /// Reaction products and their respective stoichiometric coefficients.
    products: Vec<(i32, Chem)>,
    /// Reaction rate.
    rate: Rate,
}

impl Reaction {
    access!(reactants, Vec<(i32, Chem)>);
    access!(products, Vec<(i32, Chem)>);
    access!(rate, Rate);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(reactants: Vec<(i32, Chem)>, products: Vec<(i32, Chem)>, rate: Rate) -> Self {
        debug_assert!(!reactants.is_empty());

        Self {
            reactants,
            products,
            rate,
        }
    }
}
