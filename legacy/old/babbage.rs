//! Datacube manipulation.

use arctk::{
    args,
    file::{Build, Load},
    report,
    sim::babbage::{Operation, OperationBuilder},
    util::{banner, dir},
};
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    /// Operation to perform.
    op: OperationBuilder,
}

// use std::fmt::Display;
// impl std::fmt::Display for Parameters {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
//         write_report!(Self::fmt, self.op, "operation")
//     }
// }

/// Main function.
pub fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    banner::title("BABBAGE", term_width);

    let (params_path, in_dir, out_dir) = init(term_width);

    let params = input(term_width, &in_dir, &params_path);

    let op = build(term_width, &in_dir, params);

    banner::section("Operation", term_width);
    op.run(&out_dir)
        .expect("Operation failed... we'll get 'em next time.");

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(params_path.display(), "parameters path");

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (params_path, in_dir, out_dir)
}

/// Load the input files.
#[allow(clippy::let_and_return)]
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");

    params
}

/// Build instances.
#[allow(clippy::let_and_return)]
fn build(term_width: usize, in_dir: &Path, params: Parameters) -> Operation {
    banner::section("Building", term_width);
    banner::sub_section("Operation", term_width);
    let op = params.op.build(in_dir).expect("Failed to build operation.");

    op
}
