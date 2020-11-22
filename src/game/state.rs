//! Game state.

use crate::game::*;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::{Builder, Join, RunNow, World, WorldExt};

/// Game state.
pub struct State {
    /// Entity-Component-System
    ecs: World,
}

impl State {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<LeftWalker>();
        ecs.register::<Player>();

        Self { ecs }
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
            pos.x = (pos.x + dx).max(0).min(79);
            pos.y = (pos.y + dy).max(0).min(49);
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

impl Default for State {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new()
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
