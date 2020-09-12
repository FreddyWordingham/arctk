//! System settings structure.

use crate::{clone, display_field};
use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable symbols settings structure.
#[load]
pub struct Symbols {
    /// Player character.
    player: char,
}

impl Symbols {
    clone!(player, char);
}

impl Display for Symbols {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "player", self.player)
    }
}
