//! Reactor linking structure.

use crate::{
    chem::{Rate, Reaction, ReactionLinker},
    ord::Register,
};
use arctk_attr::load;
use ndarray::{Array1, Array2};

/// Reactor linking structure.
#[load]
pub struct ReactorLinker {
    /// Reactions.
    reacts: Vec<ReactionLinker>,
}
