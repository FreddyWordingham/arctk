//! Rendering functions.

use crate::{game::torus::Entity, X, Y};

use tcod::console::{BackgroundFlag, Console, Root};

/// Render the current state of the game.
pub fn entities(window: &mut Root, player: &Entity) {
    window.set_default_foreground(player.draw().col());
    window.clear();

    let px = player.pos()[X];
    let py = window.height() - player.pos()[Y];
    window.put_char(px, py, player.draw().sym(), BackgroundFlag::None);
    window.flush();
}
