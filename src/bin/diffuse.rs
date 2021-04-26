//! Diffusion simulation binary.
//! Compute the time evolution of a volume of diffusing species.

use arctk::{
    args,
    fs::{File, Load},
    ord::Build,
    report,
    sim::diffuse::{run, Input, Parameters, ParametersBuilderLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
        fmt::term,
        Analyze,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Backup print width if the terminal width can not be determined.
const BACKUP_TERM_WIDTH: usize = 80;

/// Main simulation function.
fn main() {
    let term_width = term::width(BACKUP_TERM_WIDTH);
    title(term_width, "Diffuse");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let sett = params.sett;
    report!(sett, "settings");
    let grid = params.grid;
    report!(grid, "measurement grid");
    let coeffs = params.coeffs;
    report!(coeffs.display(), "diffusion coefficients (m^2/s)");
    let sources = params.sources;
    report!(sources.display(), "sources/sinks");
    let values = params.init;
    report!(values.display(), "initial values");

    sub_section(term_width, "Input");
    let input = Input::new(&coeffs, &sources, &grid, &sett);
    report!(input, "input");

    section(term_width, "Running");
    run::multi_thread(&input, values, &out_dir).expect("Failed to run diffuse simulation.");

    section(term_width, "Finished");
}

/// Initialise the input arguments.
fn initialisation(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    section(term_width, "Initialisation");
    sub_section(term_width, "args");
    args!(
        bin_path: PathBuf;
        output_dir: PathBuf;
        input_dir: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(output_dir.display(), "relative output path");
    report!(input_dir.display(), "relative input path");
    report!(params_path.display(), "parameters");

    sub_section(term_width, "directories");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join(input_dir)), Some(cwd.join(output_dir)))
        .expect("Failed to initialise directories.");
    report!(out_dir.display(), "output directory");
    report!(in_dir.display(), "input directory");

    (in_dir, out_dir, params_path)
}

/// Load the required files and form the input parameters.
fn load_parameters(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    section(term_width, "Parameters");
    sub_section(term_width, "Loading");
    let builder = ParametersBuilderLoader::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.")
        .load(&in_dir)
        .expect("Failed to load parameter resource files.");
    report!(builder, "builder");

    sub_section(term_width, "Building");
    let params = builder.build();
    report!(params, "parameters");

    params
}
