//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::{
    args,
    file::{Build, Load},
    report,
    // sim::babbage::OperationBuilder,
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    // /// Operation to perform.
// op: OperationBuilder,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Babbage");

    let (in_dir, _out_dir, params_path) = initialisation(term_width);
    let _params = input(term_width, &in_dir.join(&params_path));

    section(term_width, "Finished");
}

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

fn input(term_width: usize, params_path: &Path) -> Parameters {
    let params = Parameters::load(params_path).expect("Failed to load parameters file.");

    params
}
