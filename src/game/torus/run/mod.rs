//! Game running module.

use crate::{
    game::torus::{Coor2, Entity, Input, Move2, Symbols},
    X, Y,
};

use tcod::colors::WHITE;
use tcod::console::{BackgroundFlag, Console, FontLayout, FontType, Root};
use tcod::input::Key;

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

    let mut player = Entity::new(Coor2::new(
        input.sys.resolution()[X] / 2,
        input.sys.resolution()[Y] / 2,
    ));

    while !window.window_closed() {
        render(&mut window, input.symbols, &player);

        let key = window.wait_for_keypress(true);
        handle_keys(key, &mut player);
    }
}

/// Render the current state of the game.
fn render(window: &mut Root, symbols: &Symbols, player: &Entity) {
    window.set_default_foreground(WHITE);
    window.clear();

    let px = player.pos()[X];
    let py = window.height() - player.pos()[Y];
    window.put_char(px, py, symbols.player(), BackgroundFlag::None);
    window.flush();
}

/// Handle key input.
fn handle_keys(key: Key, player: &mut Entity) {
    match key {
        Key { printable: 'w', .. } => player.travel(Move2::new(0, 1)),
        Key { printable: 's', .. } => player.travel(Move2::new(0, -1)),
        Key { printable: 'd', .. } => player.travel(Move2::new(1, 0)),
        Key { printable: 'a', .. } => player.travel(Move2::new(-1, 0)),
        _ => {}
    }
}
