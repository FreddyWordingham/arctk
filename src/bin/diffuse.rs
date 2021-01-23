//! Diffusion simulation binary.
//! Compute the time evolution of a volume of diffusing species.

use arctk::{
    args,
    fs::{File, Load, Save},
    ord::{Link, Register},
    report,
    sim::diffuse::{Parameters, ParametersLoader},
    // sim::diffuse::{run, Input, Parameters, ParametersLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Main simulation function.
fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Diffuse");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    // section(term_width, "Input");
    // sub_section(term_width, "Reconstruction");
    // let sett = params.sett;
    // report!(sett, "settings");

    // sub_section(term_width, "Registration");
    // let spec_reg = Register::new(params.reactor.requires());
    // report!(spec_reg, "species register");

    // sub_section(term_width, "Linking");
    // let concs = params
    //     .concs
    //     .link(spec_reg.set())
    //     .expect("Failed to link species to initial concentrations.");
    // report!(concs, "concs");

    // let reactor = params
    //     .reactor
    //     .link(spec_reg.set())
    //     .expect("Failed to link species to reactor.");
    // report!(reactor, "reactor");

    // sub_section(term_width, "Input");
    // let input = Input::new(&spec_reg, &reactor, &sett);
    // report!(input, "input");

    // section(term_width, "Running");
    // let data = run::single_thread(concs, &input).expect("Failed to run cartographer.");

    // section(term_width, "Saving");
    // report!(data, "data");
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
    let params = ParametersLoader::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.")
        .load(&in_dir)
        .expect("Failed to load parameter resource files.");
    report!(params, "parameters");

    params
}
