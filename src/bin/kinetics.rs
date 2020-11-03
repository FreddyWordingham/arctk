//! Chemical reaction simulation.

use arctk::util::*;
use arctk::*;
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    // /// Reaction simulation settings.
    // sett: kinetics::Settings,
    // /// Reactions.
    // reactions: Vec<kinetics::ReactionBuilder>,
    // /// Initial concentrations.
    // concs: chem::ConcBuilder,
    /// Total time.
    total_time: f64,
}

/// Compute the change in concentration,
/// of a set of chemical species,
/// given a set of reaction equations,
/// and their respective reaction rates,
/// from an initial set of concentrations.
pub fn main() {
    let term_width = term::width().unwrap_or(80);

    banner::title("Kinetics", term_width);

    banner::section("Initialisation", term_width);
    let (params_path, in_dir, out_dir) = init(term_width);

    banner::section("Input", term_width);
    // let params = input(&in_dir, &params_path);

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
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

// /// Load the input files.
// fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> FlaskBuilder {
//     banner::sub_section("Parameters", term_width);
//     let path = in_dir.join(params_path);

//     FlaskBuilder::load(&path).expect("Failed to load parameters file.")
// }
