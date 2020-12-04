//! Reaction linker structure.

use arctk_attr::load;

/// Reaction linker.
#[load]
pub struct ReactionLinker {
    // /// Reaction rate.
    // rate: RateLinker,
    /// List of each reactant species and its stoichiometric coefficient.
    reactants: Vec<(String, f64)>,
    /// List of each product species and its stoichiometric coefficient.
    products: Vec<(String, f64)>,
}
