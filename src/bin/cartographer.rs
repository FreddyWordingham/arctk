//! Volumetric mapping binary.
//! Produce datacubes mapping the location of each material.

use arctk::{
    args,
    file::{Build, Load, Save},
    geom::Tree,
    ord::Link,
    util::{
        banner::{section, title},
        dir,
    },
};
use arctk_attr::input;
use std::{env::current_dir, path::PathBuf};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Cartographer");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let builder = ParametersBuilder::load(&in_dir.join(params_path))
        .expect("Failed to load parameters file.");

    section(term_width, "Building");
    let setup = builder
        .build(&in_dir)
        .expect("Failed to construct Babbage operation.");

    section(term_width, "Linking");

    section(term_width, "Mapping");
    // let output = single_thread(engine, &input).expect("Failed to run mapping");
    let output = multi_thread(engine, &input).expect("Failed to run mapping");
    output.save(&out_dir).expect("Failed to save output data.");

    section(term_width, "Finished");
}
