//! Datacube manipulation engine binary.

use arctk::{
    args,
    file::Load,
    game::{Map, State},
    ord::{X, Y},
    util::{
        banner::{section, title},
        dir,
    },
};
use arctk_attr::input;
use rltk::{main_loop, RltkBuilder};
use std::{env::current_dir, path::PathBuf};

// Input parameters.
#[input]
struct Parameters {
    /// Map size.
    res: [usize; 2],
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Wonder");

    section(term_width, "Initialisation");
    args!(
        bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, _out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");

    section(term_width, "Running");
    game(&params);

    section(term_width, "Finished");
}

/// Run the main game loop.
fn game(params: &Parameters) {
    let context = RltkBuilder::simple(params.res[X], params.res[Y])
        .expect("Failed to build console of the given dimensions.")
        .with_title("Roguelike - Wonder")
        .build()
        .expect("Failed to build RLTK window.");
    let mut gs = State::new(Map::new_forest(params.res));

    gs.add_player((params.res[X] / 2) as i32, (params.res[Y] / 3) as i32);
    for i in 0..10 {
        gs.add_enemy(i as i32 * 3, (2 * params.res[Y] as i32) / 3);
    }

    main_loop(context, gs).expect("Failed to run the main game loop.")
}
