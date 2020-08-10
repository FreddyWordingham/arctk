//! Diffusion programme.

use arctk::*;
use attr::input;
use ndarray::Array3;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Diffusion simulation settings.
    sett: diffusion::Settings,
}

/// Main function.
pub fn main() {
    banner::title("Diffusion");

    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (diff_sett, concs, coeffs) = build(&in_dir, params);
    let input = diffusion::Scene::new(&diff_sett, &coeffs);
    let data = diffusion::run::multi_thread(&input, concs);

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
fn build(_in_dir: &Path, params: Parameters) -> (diffusion::Settings, Array3<f64>, Array3<f64>) {
    banner::section("Building");
    banner::sub_section("Diffusion Settings");
    let diff_sett = params.sett;
    report!("Diffusion settings", &diff_sett);

    banner::sub_section("Concentration");
    let res = [61, 61, 61];
    let mut concs = Vec::with_capacity(res[X] * res[Y] * res[Z]);
    for _ in 0..res[X] {
        for _ in 0..res[Y] {
            for _ in 0..res[Z] {
                concs.push(0.0);
            }
        }
    }
    let mut concs =
        Array3::from_shape_vec(res, concs).expect("Could not create concentration array.");
    concs[[30, 30, 30]] = 1.0;

    banner::sub_section("Concentration");
    let res = [61, 61, 61];
    let mut coeffs = Vec::with_capacity(res[X] * res[Y] * res[Z]);
    for _ in 0..res[X] {
        for _ in 0..res[Y] {
            for _ in 0..res[Z] {
                coeffs.push(1e-3);
            }
        }
    }
    let coeffs = Array3::from_shape_vec(res, coeffs).expect("Could not create coefficient array.");

    (diff_sett, concs, coeffs)
}
