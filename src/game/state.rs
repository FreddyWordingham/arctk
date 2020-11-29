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
    pub fn new(map: Map) -> Self {
        let mut ecs = World::new();

        // Systems.
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<LeftWalker>();
        ecs.register::<Player>();
        ecs.register::<Viewshed>();

        // Resources.
        ecs.insert(map);

        Self { ecs }
    }

    /// Run the systems.
    #[inline]
    pub fn run_systems(&mut self) {
        let mut vis = Visibility::new();
        vis.run_now(&self.ecs);

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
            .with(Viewshed::new(8))
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

        let map = self.ecs.fetch::<Map>();
        let [width, height] = map.res();

        for (_player, pos) in (&mut players, &mut positions).join() {
            let destination = [(pos.p.x + dx) as usize, (pos.p.y + dy) as usize];

            if map.tiles[destination].is_passable() {
                pos.p.x = (pos.p.x + dx).max(0).min(width as i32);
                pos.p.y = (pos.p.y + dy).max(0).min(height as i32);
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
                VirtualKeyCode::A | VirtualKeyCode::Left => self.try_move_player(-1, 0),
                VirtualKeyCode::D | VirtualKeyCode::Right => self.try_move_player(1, 0),
                VirtualKeyCode::W | VirtualKeyCode::Up => self.try_move_player(0, 1),
                VirtualKeyCode::S | VirtualKeyCode::Down => self.try_move_player(0, -1),
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

        let map = self.ecs.fetch::<Map>();

        ctx.cls();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let [_width, height] = map.res();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(
                pos.p.x,
                height as i32 - pos.p.y - 1,
                render.fg,
                render.bg,
                render.glyph,
            );
        }
    }
}
