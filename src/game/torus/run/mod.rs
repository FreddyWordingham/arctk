//! Game running module.

use crate::{game::torus::Input, X, Y};

use tcod::colors::*;
use tcod::console::*;

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

    let player = Entity::new([1, 1]);

    while !window.window_closed() {
        window.set_default_foreground(WHITE);
        window.clear();
        window.put_char(
            player.pos[X],
            player.pos[Y],
            input.symbols.player(),
            BackgroundFlag::None,
        );
        window.flush();
        window.wait_for_keypress(true);
    }
}

pub struct Entity {
    /// Position.
    pub pos: [i32; 2],
}

impl Entity {
    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub fn new(pos: [i32; 2]) -> Self {
        Self { pos }
    }
}
