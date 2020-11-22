//! Game state.

use crate::{
    clone,
    game::*,
    ord::{X, Y},
};
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::{Builder, Join, RunNow, World, WorldExt};

// Minimum window resolution in all dimensions.
const MIN_WINDOW_RES: i32 = 32;

/// Game state.
pub struct State {
    /// Window resolution.
    res: [i32; 2],
    /// Entity-Component-System
    ecs: World,
}

impl State {
    clone!(res, [i32; 2]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [i32; 2]) -> Self {
        debug_assert!(res[X] > MIN_WINDOW_RES);
        debug_assert!(res[Y] > MIN_WINDOW_RES);

        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<LeftWalker>();
        ecs.register::<Player>();

        Self { res, ecs }
    }

    /// Run the systems.
    #[inline]
    pub fn run_systems(&mut self) {
        let mut walk = WalkLeft::new();
        walk.run_now(&self.ecs);

        self.ecs.maintain();
    }

    /// Add an player entity.
    #[inline]
    pub fn add_player(&mut self, x: i32, y: i32) {
        self.ecs
            .create_entity()
            .with(Position::new(x, y))
            .with(Renderable::new(
                'O',
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
            ))
            .with(Player::new())
            .build();
    }

    /// Add an enemy entity.
    #[inline]
    pub fn add_enemy(&mut self, x: i32, y: i32) {
        self.ecs
            .create_entity()
            .with(Position::new(x, y))
            .with(Renderable::new(
                '*',
                RGB::named(rltk::RED),
                RGB::named(rltk::BLACK),
            ))
            .with(LeftWalker::new())
            .build();
    }

    /// Try to move the player.
    #[inline]
    fn try_move_player(&mut self, dx: i32, dy: i32) {
        let mut positions = self.ecs.write_storage::<Position>();
        let mut players = self.ecs.write_storage::<Player>();

        for (_player, pos) in (&mut players, &mut positions).join() {
            pos.x = (pos.x + dx).max(0).min(self.res[X]);
            pos.y = (pos.y + dy).max(0).min(self.res[Y]);
        }
    }

    /// Handle keypresses.
    #[inline]
    fn handle_input(&mut self, ctx: &mut Rltk) {
        // Player movement.
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::A => self.try_move_player(-1, 0),
                VirtualKeyCode::D => self.try_move_player(1, 0),
                VirtualKeyCode::W => self.try_move_player(0, -1),
                VirtualKeyCode::S => self.try_move_player(0, 1),
                _ => {}
            },
        }
    }
}

impl GameState for State {
    #[inline]
    fn tick(&mut self, ctx: &mut Rltk) {
        println!("Tick!");

        self.handle_input(ctx);
        self.run_systems();

        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
