//! Data cube manipulation programme.

use arctk::*;
use arctk_attr::input;
use ndarray::Array3;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Path to the data cubes of interest.
    data: Vec<PathBuf>,
}

/// Main function.
pub fn main() {
    banner::title("Blitzer");

    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let data = build(&in_dir, params);

    banner::section("Manipulating");
    let out = (&data[0] * 1.0e-3) + 1.0e-4;

    banner::section("Saving");
    out.save(&out_dir.join("output.nc"))
        .expect("Failed to save output data.");

    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line arguments");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!("binary path", bin_path.display());
    report!("parameters path", params_path.display());

    banner::sub_section("Directories");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let exec_name = exec::name().expect("Could not determine executable name.");

    let (in_dir, out_dir) = dir::io_dirs(
        Some(cwd.join("input").join(exec_name.clone())),
        // Some(
        //     cwd.join("output").join(exec_name).join(
        //         chrono::offset::Local::now()
        //             .format("%Y%m%d%H%M%S")
        //             .to_string(),
        //     ),
        // ),
        Some(cwd.join("output").join(exec_name)),
    )
    .expect("Could not initialise directories");
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input");
    banner::sub_section("Parameters");
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Could not load parameters file")
}

/// Build instances.
fn build(in_dir: &Path, params: Parameters) -> Vec<Array3<f64>> {
    banner::section("Building");
    banner::sub_section("Data cubes");

    let mut data = Vec::with_capacity(params.data.len());
    for p in params.data {
        let path = in_dir.join(p);
        println!("Loading data: {}", path.display());
        data.push(Array3::load(&path).expect("Failed to load data cube."));
    }

    data
}
