//! Datacube manipulation engine binary.

use arctk::{
    args,
    game::State,
    util::{
        banner::{section, title},
        dir,
    },
};
use rltk::{main_loop, RltkBuilder};
use std::{env::current_dir, path::PathBuf};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Wonder");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (_in_dir, _out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Running");
    game();

    section(term_width, "Finished");
}

/// Run the main game loop.
fn game() {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike - Wonder")
        .build()
        .expect("Failed to build RLTK window.");
    let mut gs = State::new();

    gs.add_player(40, 25);
    for i in 0..10 {
        gs.add_enemy(i * 7, 20);
    }

    main_loop(context, gs).expect("Failed to run the main game loop.")
}
