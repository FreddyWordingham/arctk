//! Diffusion programme.

use arctk::*;
use arctk_attr::input;
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Diffusion simulation settings.
    sett: diffusion::Settings,
    /// Path to the diffusion coefficient data cube.
    coeffs: PathBuf,
    /// Path to the initial concentration data cube.
    init_concs: PathBuf,
}

/// Main function.
pub fn main() {
    banner::title("Diffusion");

    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (diff_sett, coeffs, mut concs) = build(&in_dir, params);

    println!("-> {}", concs.sum());
    let path = out_dir.join("diffusion_0.nc");
    println!("Saving: {}", path.display());
    concs.save(&path).expect("Failed to save diffusion step.");

    for i in 0..diff_sett.num_dumps() {
        concs = diffusion::run::single_thread(&diff_sett, &coeffs, concs);

        println!("-> {}", concs.sum());
        let path = out_dir.join(&format!("diffusion_{}.nc", i + 1));
        println!("Saving: {}", path.display());
        concs.save(&path).expect("Failed to save diffusion step.");
    }

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
fn build(in_dir: &Path, params: Parameters) -> (diffusion::Settings, Array3<f64>, Array3<f64>) {
    banner::section("Building");
    banner::sub_section("Diffusion Settings");
    let diff_sett = params.sett;
    report!("Diffusion settings", &diff_sett);

    banner::sub_section("Coefficents");
    let coeffs =
        Array3::load(&in_dir.join(params.coeffs)).expect("Failed to load coefficient array.");
    report!(
        "Minimum coefficient",
        coeffs
            .min()
            .expect("Failed to determine minimum diffusion coefficient."),
        "X"
    );
    report!(
        "Maximum coefficient",
        coeffs
            .max()
            .expect("Failed to determine maximum diffusion coefficient."),
        "X"
    );

    banner::sub_section("Concentration");
    let concs = Array3::load(&in_dir.join(params.init_concs))
        .expect("Failed to load initial concentration array.");
    report!(
        "Minimum concentration",
        concs
            .min()
            .expect("Failed to determine minimum diffusion concentration."),
        "X"
    );
    report!(
        "Maximum concentration",
        concs
            .max()
            .expect("Failed to determine maximum diffusion concentration."),
        "X"
    );

    (diff_sett, coeffs, concs)
}
