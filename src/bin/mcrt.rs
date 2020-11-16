//! Monte-Carlo radiative transfer simulation binary.
//! Compute the radiative field for a given set of setup and light source.

use arctk::{
    args,
    file::{Build, Load, Save},
    geom::Tree,
    ord::Link,
    sim::mcrt::{multi_thread, Input, ParametersBuilder},
    util::{
        banner::{section, title},
        dir,
    },
};
use std::{env::current_dir, path::PathBuf};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "MCRT");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let builder = ParametersBuilder::load(&in_dir.join(params_path))
        .expect("Failed to load parameters file.");

    section(term_width, "Building");
    let setup = builder
        .build(&in_dir)
        .expect("Failed to construct builder structure.");

    section(term_width, "Linking");
    let mats = setup.mats;
    let attrs = setup.attrs.link(&mats).expect("Material link failure.");
    let surfs = setup.surfs.link(&attrs).expect("Surface link failure.");
    let light = setup.light;
    let tree = Tree::new(&setup.tree, &surfs);
    let grid = setup.grid;
    let sett = setup.sett.link(&mats).expect("Material link Failure.");
    let engine = setup.engine;
    let input = Input::new(&mats, &attrs, &light, &tree, &grid, &sett);

    section(term_width, "Simulation");
    // let output = single_thread(engine, &input).expect("Failed to run simulation");
    let output = multi_thread(engine, &input).expect("Failed to run simulation");
    output.save(&out_dir).expect("Failed to save output data.");

    section(term_width, "Finished");
}
