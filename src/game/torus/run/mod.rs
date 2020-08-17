//! Game running module.

pub mod render;

pub use self::render::*;

use crate::{
    game::torus::{Coor2, Draw, Entity, Input, Move2},
    X, Y,
};

use tcod::colors::WHITE;
use tcod::console::{FontLayout, FontType, Root};
use tcod::input::{Key, KeyCode};

/// Start a new game.
#[inline]
pub fn start(input: &Input) {
    println!("Hello world!");

    let mut window = Root::initializer()
        .font(input.sys.font(), FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(input.sys.resolution()[X], input.sys.resolution()[Y])
        .title(input.sys.title())
        .fullscreen(true)
        .init();
    tcod::system::set_fps(input.sys.fps());

    let mut ents = Vec::new();
    ents.push(Entity::new(
        Coor2::new(input.sys.resolution()[X] / 2, input.sys.resolution()[Y] / 2),
        Draw::new(input.symbols.player(), WHITE),
    ));
    let mut player = &mut ents[0];

    while !window.window_closed() {
        render::entities(&mut window, player);

        let key = window.wait_for_keypress(true);
        if handle_keys(key, &mut player) {
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
