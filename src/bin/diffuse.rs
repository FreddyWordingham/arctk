//! Diffusion calculation program.

use arctk::{
    args,
    file::{Build, Load, Redirect, Save},
    geom::{Grid, GridBuilder},
    sim::diffuse::{Cloud, Data, Settings},
    util::{banner, dir},
};
use arctk_attr::input;
use ndarray::Array3;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    /// Total wall clock time.
    total_time: f64,
    /// Intermediate output steps.
    steps: Option<u64>,
    /// Initial state.
    init: PathBuf,
    /// Coefficient map.
    coeffs: PathBuf,
    /// Integration settings.
    sett: Redirect<Settings>,
    /// Sampling grid settings.
    grid: Redirect<GridBuilder>,
}

/// Main function.
pub fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    banner::title("DIFFUSE", term_width);

    let (params_path, in_dir, out_dir) = init(term_width);

    let params = input(term_width, &in_dir, &params_path);

    let (total_time, steps, init, coeffs, sett, grid_sett) = build(term_width, &in_dir, params);

    let grid = grow(term_width, grid_sett);

    let cloud = Cloud::new(&coeffs, &sett, &grid);

    pre_analysis(term_width, &init);

    let output = sim_loop(term_width, &out_dir, total_time, steps, init, &cloud);

    post_analysis(term_width, &output);

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
#[allow(clippy::type_complexity)]
fn build(
    term_width: usize,
    in_dir: &Path,
    params: Parameters,
) -> (f64, u64, Array3<f64>, Array3<f64>, Settings, GridBuilder) {
    banner::section("Building", term_width);
    banner::sub_section("Total wall Clock Time", term_width);
    let total_time = params.total_time;

    banner::sub_section("Intermediate steps", term_width);
    let steps = params.steps.unwrap_or(1);

    banner::sub_section("Initial Values", term_width);
    let init =
        Array3::load(&in_dir.join(params.init)).expect("Failed to load initial value array.");

    banner::sub_section("Coefficents", term_width);
    let coeffs =
        Array3::load(&in_dir.join(params.coeffs)).expect("Failed to load coefficient value array.");

    banner::sub_section("Integration Settings", term_width);
    let sett = params
        .sett
        .build(in_dir)
        .expect("Failed to redirect integration settings.");

    banner::sub_section("Grid Settings", term_width);
    let grid_sett = params
        .grid
        .build(in_dir)
        .expect("Failed to redirect grid settings.");

    (total_time, steps, init, coeffs, sett, grid_sett)
}

/// Grow domains.
fn grow<'a>(term_width: usize, grid_sett: GridBuilder) -> Grid {
    banner::section("Growing", term_width);

    banner::sub_section("Regular Grid", term_width);
    let grid = grid_sett.build();

    grid
}

/// Review the input data.
fn pre_analysis(term_width: usize, values: &Array3<f64>) {
    banner::section("Pre-Analysis", term_width);
    println!("{:>32} : {}", "total", values.sum());
}

/// Run the repeating steps of the simulation.
fn sim_loop(
    term_width: usize,
    out_dir: &Path,
    total_time: f64,
    steps: u64,
    init: Array3<f64>,
    cloud: &Cloud,
) -> Data {
    banner::section("Simulating", term_width);
    let dt = total_time / steps as f64;

    let mut data = Data::new(init);
    for _ in 0..steps {
        let output = cloud.sim(dt, data);
        output.save(&out_dir).expect("Failed to save output data.");
        data = output;
    }

    data
}

/// Review the output data.
fn post_analysis(term_width: usize, output: &Data) {
    banner::section("Post-Analysis", term_width);

    let values = &output.values;
    println!("{:>32} : {}", "total", values.sum());
}
