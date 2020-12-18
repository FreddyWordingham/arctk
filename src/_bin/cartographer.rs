//! Volumetric mapping binary.
//! Produce datacubes mapping the location of each material.

use arctk::{
    args,
    file::{Build, Load},
    report,
    sim::cartographer::ParametersBuilder,
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Cartographer");

    let (in_dir, _out_dir, params_path) = initialisation(term_width);
    let _ = input(term_width, &in_dir, &params_path);

    section(term_width, "Finished");
}

/// Initialise the input arguments.
fn initialisation(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    section(term_width, "Initialisation");
    sub_section(term_width, "args");
    args!(
        bin_path: PathBuf;
        input_dir: PathBuf;
        output_dir: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(input_dir.display(), "relative input path");
    report!(output_dir.display(), "relative output path");
    report!(params_path.display(), "parameters");

    sub_section(term_width, "directories");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join(input_dir)), Some(cwd.join(output_dir)))
        .expect("Failed to initialise directories.");
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (in_dir, out_dir, params_path)
}

/// Retrieve the input parameters file structure.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> () {
    section(term_width, "Input");
    sub_section(term_width, "Loading");
    let params = ParametersBuilder::load(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.");
    report!(params, "parameters");

    sub_section(term_width, "Building");
    let build = params.build(in_dir).expect("Failed to build parameters.");
    report!(build, "build");
    // let op = params
    //     .op
    //     .build(&in_dir)
    //     .expect("Failed to construct Babbage operation.");
    // report!(op, "operation");

    ()
}
