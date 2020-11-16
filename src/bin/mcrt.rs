//! Monte-Carlo radiative transfer simulation binary.
//! Compute the radiative field for a given set of setup and light source.

use arctk::{
    args,
    file::{Build, Load, Save},
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

    section(term_width, "Setup");
    let (params, _cat) = setup.setup();
    let tree = params.grow();
    let input = Input::new(
        &tree,
        &params.grid,
        &params.sett,
        &params.mats,
        &params.attrs,
        &params.light,
    );

    section(term_width, "Simulation");
    // let output = single_thread(params.engine, &input).expect("Failed to run simulation");
    let output = multi_thread(params.engine, &input).expect("Failed to run simulation");
    output.save(&out_dir).expect("Failed to save output data.");

    section(term_width, "Finished");
}
