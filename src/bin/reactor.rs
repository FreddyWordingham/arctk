//! Reaction-diffusion simulation binary.
//! Compute the time evolution of a set of reacting and diffusing species.

use arctk::{
    args,
    fs::{File, Load, Save},
    ord::{Build, Link, Register, X, Y, Z},
    report,
    sim::reactor::{Parameters, ParametersBuilderLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
        fmt::term,
    },
};
use ndarray::Array4;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Backup print width if the terminal width can not be determined.
const BACKUP_TERM_WIDTH: usize = 80;

/// Main simulation function.
fn main() {
    let term_width = term::width(BACKUP_TERM_WIDTH);
    title(term_width, "Reactor");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let sett = params.sett;
    let grid = params.grid;

    sub_section(term_width, "Registration");
    let spec_reg = Register::new(params.reactor.requires());
    report!(spec_reg, "species register");

    sub_section(term_width, "Linking");
    let shape = [spec_reg.len(), grid.res()[X], grid.res()[Y], grid.res()[Z]];
    let mut values: Array4<f64> = Array4::zeros(shape);
    let mut coeffs: Array4<f64> = Array4::zeros(shape);
    for (name, (vs, cs)) in params.values_coeffs.map() {
        let index = spec_reg.set().map()[name];

        for zi in 0..grid.res()[Z] {
            for yi in 0..grid.res()[Y] {
                for xi in 0..grid.res()[X] {
                    values[[index, xi, yi, zi]] = vs[[xi, yi, zi]];
                    coeffs[[index, xi, yi, zi]] = cs[[xi, yi, zi]];
                }
            }
        }
    }

    // let concs = params
    //     .init
    //     .link(spec_reg.set())
    //     .expect("Failed to link species to initial concentrations.");
    // report!(concs, "initial concentrations");

    // let reactor = params
    //     .reactor
    //     .link(spec_reg.set())
    //     .expect("Failed to link species to reactor.");
    // report!(reactor, "reactor");

    // sub_section(term_width, "Input");
    // let input = Input::new(&spec_reg, &reactor, &sett);
    // report!(input, "input");

    // section(term_width, "Running");
    // let data = run(concs, &input).expect("Failed to run flask simulation.");

    // section(term_width, "Saving");
    // // report!(data, "data");
    // data.save(&out_dir.join("concs.csv"))
    //     .expect("Failed to save output data.");

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
    let params = ParametersBuilderLoader::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.")
        .load(&in_dir)
        .expect("Failed to load parameter resource files.");
    report!(params, "parameters");

    sub_section(term_width, "Building");
    let params = params.build();
    report!(params, "parameters");

    params
}
