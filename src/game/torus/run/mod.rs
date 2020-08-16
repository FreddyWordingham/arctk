//! Game running module.

use crate::{
    game::torus::{Input, Symbols},
    X, Y,
};

use tcod::colors::WHITE;
use tcod::console::{BackgroundFlag, Console, FontLayout, FontType, Root};
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
        .init();
    tcod::system::set_fps(input.sys.fps());

    let mut player = Entity::new([input.sys.resolution()[X] / 2, input.sys.resolution()[Y] / 2]);

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
    window.put_char(
        player.pos[X],
        player.pos[Y],
        symbols.player(),
        BackgroundFlag::None,
    );
    window.flush();
}

/// Handle key input.
fn handle_keys(key: Key, player: &mut Entity) -> bool {
    match key {
        // movement keys
        Key {
            code: KeyCode::Up, ..
        } => player.pos[Y] -= 1,
        Key {
            code: KeyCode::Down,
            ..
        } => player.pos[Y] += 1,
        Key {
            code: KeyCode::Left,
            ..
        } => player.pos[X] -= 1,
        Key {
            code: KeyCode::Right,
            ..
        } => player.pos[X] += 1,
        _ => {}
    }

    false
}

/// Game entity.
pub struct Entity {
    /// Position.
    pub pos: [i32; 2],
}

impl Entity {
    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub const fn new(pos: [i32; 2]) -> Self {
        Self { pos }
    }
}
