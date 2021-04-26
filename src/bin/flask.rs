//! Chemical network simulation binary.
//! Compute the time evolution of a set of reacting species.

use arctk::{
    args,
    data::Table,
    fs::{File, Load, Save},
    ord::{Link, Register},
    report,
    sim::flask::{run, Input, Parameters, ParametersLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
        fmt::term,
    },
};
use ndarray::Array2;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Backup print width if the terminal width can not be determined.
const BACKUP_TERM_WIDTH: usize = 80;

/// Main simulation function.
fn main() {
    let term_width = term::width(BACKUP_TERM_WIDTH);
    title(term_width, "Flask");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let sett = params.sett;
    report!(sett, "settings");

    sub_section(term_width, "Registration");
    let mut spec_names = params.reactor.requires();
    spec_names.append(&mut params.init.requires());
    spec_names.append(&mut params.sources.requires());
    let spec_reg = Register::new(spec_names);
    report!(spec_reg, "species register");

    sub_section(term_width, "Linking");
    let values = params
        .init
        .link(spec_reg.set())
        .expect("Failed to link species to initial values.");
    report!(values, "initial values");
    let sources = params
        .sources
        .link(spec_reg.set())
        .expect("Failed to link species to sources.");
    report!(sources, "sources/sinks");

    let reactor = params
        .reactor
        .link(spec_reg.set())
        .expect("Failed to link species to reactor.");
    report!(reactor, "reactor");

    sub_section(term_width, "Input");
    let input = Input::new(&spec_reg, &sources, &reactor, &sett);
    report!(input, "input");

    section(term_width, "Running");
    let data = run(values, &input).expect("Failed to run flask simulation.");

    section(term_width, "Saving");
    save(&input, data, &out_dir);

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
    let params = ParametersLoader::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.")
        .load(&in_dir)
        .expect("Failed to load parameter resource files.");
    report!(params, "parameters");

    params
}

/// Save the output data.
fn save(input: &Input, data: Array2<f64>, out_dir: &Path) {
    let names = input.specs.names_list();
    let mut headings = Vec::with_capacity(1 + names.len());
    headings.push("time".to_string());
    for name in &names {
        headings.push(name.as_string());
    }

    let table = Table::new_from_array(headings, &data);
    table
        .save(&out_dir.join("values.csv"))
        .expect("Failed to save output data.");
}
