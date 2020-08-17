//! Game running module.

use crate::{
    game::torus::{Coor2, Draw, Entity, Input, Move2, Window},
    X, Y,
};

use tcod::{
    colors::WHITE,
    input::{Key, KeyCode},
};

/// Start a new game.
#[inline]
pub fn start(input: &Input) {
    let mut window = Window::new(
        input.sys.title(),
        input.sys.font(),
        input.sys.resolution(),
        input.sys.fps(),
    );

    let mut ents = Vec::new();
    ents.push(Entity::new(
        Coor2::new(input.sys.resolution()[X] / 2, input.sys.resolution()[Y] / 2),
        input.symbols.player(),
        WHITE,
    ));

    while !window.root().window_closed() {
        window.clear();
        for ent in &ents {
            ent.draw(window.root_mut());
        }
        window.flush();

        let key = window.root_mut().wait_for_keypress(true);
        if handle_keys(key, &mut ents[0]) {
            break;
        }
    }
}

/// Handle key input.
/// Return true when the window should close.
fn handle_keys(key: Key, player: &mut Entity) -> bool {
    match key {
        Key { printable: 'w', .. } => player.travel(Move2::new(0, 1)),
        Key { printable: 's', .. } => player.travel(Move2::new(0, -1)),
        Key { printable: 'd', .. } => player.travel(Move2::new(1, 0)),
        Key { printable: 'a', .. } => player.travel(Move2::new(-1, 0)),
        Key {
            code: KeyCode::Escape,
            ..
        } => return true,
        _ => {}
    }

    false
}
