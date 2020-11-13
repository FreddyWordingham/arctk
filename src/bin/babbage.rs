//! Datacube manipulation.

use arctk::{
    args,
    file::{Build, Load},
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
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::let_and_return)]
fn build(term_width: usize, in_dir: &Path, params: Parameters) -> Operation {
    banner::section("Building", term_width);
    banner::sub_section("Operation", term_width);
    let op = params.op.build(in_dir).expect("Failed to build operation.");

    op
}
