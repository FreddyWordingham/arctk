//! Kinetics reaction simulation binary.
//! Compute the change in concentration for a given set of reactions and initial concentrations.

use arctk::{
    args,
    file::Load,
    ord::Link,
    sim::flask::ParametersBuilder,
    util::{
        banner::{section, title},
        dir,
    },
};
use std::{env::current_dir, path::PathBuf};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "FLASK");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, _out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let builder = ParametersBuilder::load(&in_dir.join(params_path))
        .expect("Failed to load parameters file.");
    let names = builder.concs.requires();
    // let concs = builder.concs.;

    println!("NAMES: {:?}", names);

    // section(term_width, "Building");
    // let setup = builder
    //     .build(&in_dir)
    //     .expect("Failed to construct builder structure.");

    // section(term_width, "Linking");
    // let mats = setup.mats;
    // let attrs = setup.attrs.link(&mats).expect("Material link failure.");
    // let input = Input::new(&mats, &attrs, &light, &tree, &grid, &sett);

    // section(term_width, "Simulation");
    // let output = single_thread(engine, &input).expect("Failed to run simulation");
    // // let output = multi_thread(engine, &input).expect("Failed to run simulation");
    // output.save(&out_dir).expect("Failed to save output data.");

    section(term_width, "Finished");
}
