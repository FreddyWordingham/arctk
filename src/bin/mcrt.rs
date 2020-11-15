//! Monte-Carlo radiative transfer simulation binary.
//! Compute the radiative field for a given set of setup and light source.

use arctk::{
    args,
    file::Load,
    geom::MeshBuilder,
    ord::Set,
    report,
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use arctk_attr::input;
use std::env::current_dir;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Surfaces.
    surfs: Set<MeshBuilder>,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title("MCRT", term_width);

    let (params_path, in_dir, _out_dir) = init(term_width);
    let params = input(term_width, &in_dir, &params_path);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    section("Initialisation", term_width);
    sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(params_path.display(), "parameters path");

    sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(None, None)
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (params_path, in_dir, out_dir)
}

/// Load the input parameters file.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    section("Input", term_width);
    sub_section("Parameters", term_width);
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");
    // report!(params, "parameters");

    params
}
