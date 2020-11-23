//! Game state.

use crate::game::*;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::{Builder, Join, RunNow, World, WorldExt};

/// Game state.
pub struct State {
    /// Map.
    map: Map,
    /// Entity-Component-System
    ecs: World,
}

impl State {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: Map) -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<LeftWalker>();
        ecs.register::<Player>();

        Self { map, ecs }
    }

    /// Run the systems.
    #[inline]
    pub fn run_systems(&mut self) {
        let [width, height] = self.map.res();
        WalkLeft::new(width as i32, height as i32).run_now(&self.ecs);

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
            let destination = [(pos.x + dx) as usize, (pos.y + dy) as usize];

            if self.map.tiles[destination].is_passable() {
                pos.x = (pos.x + dx).max(0).min(self.map.width() as i32);
                pos.y = (pos.y + dy).max(0).min(self.map.height() as i32);
            }
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
                VirtualKeyCode::W => self.try_move_player(0, 1),
                VirtualKeyCode::S => self.try_move_player(0, -1),
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
        self.map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let height = self.map.height() as i32;
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(
                pos.x,
                height - pos.y - 1,
                render.fg,
                render.bg,
                render.glyph,
            );
        }
    }
}
