//! Chemical network simulation binary.
//! Compute the time evolution of a set of reacting chemicals.

use arctk::{
    args,
    fs::{File, Load},
    ord::Build,
    report,
    sim::flask::{Parameters, ParametersLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Main simulation function.
fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Flask");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let sett = params.sett;
    report!(sett, "settings");

    sub_section(term_width, "Input");
    // let input = Input::new(&specs, &reactor, &sett);
    // report!(input, "input");

    // run(term_width, params.op, &out_dir);

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

/// Load the required files and form the input parameters.
fn load_parameters(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    section(term_width, "Parameters");
    sub_section(term_width, "Loading");
    let params = ParametersLoader::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.")
        .load(&in_dir)
        .expect("Failed to load parameter resource files.");
    report!(params, "parameters");

    params
}
