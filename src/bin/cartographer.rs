//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::{
    args,
    fs::{File, Load, Save},
    geom::Tree,
    ord::{Build, Link, Register},
    report,
    sim::cartographer::{run, Input, Parameters, ParametersBuilderLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
        fmt::term,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Backup print width if the terminal width can not be determined.
const BACKUP_TERM_WIDTH: usize = 80;

/// Main program function.
fn main() {
    let term_width = term::width(BACKUP_TERM_WIDTH);
    title(term_width, "Cartographer");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let sett = params.sett;
    report!(sett, "settings");
    let grid = params.grid;
    report!(grid, "measurement grid");

    sub_section(term_width, "Registration");
    let mat_reg = Register::new(params.attrs.requires());
    report!(mat_reg, "material register");

    sub_section(term_width, "Linking");
    let attrs = params
        .attrs
        .link(mat_reg.set())
        .expect("Failed to link attribute to surfaces.");
    report!(attrs, "attributes");
    let surfs = params
        .surfs
        .link(&attrs)
        .expect("Failed to link materials to attributes.");
    report!(surfs, "surfaces");

    sub_section(term_width, "Growing");
    let tree = Tree::new(&params.tree, &surfs);
    report!(tree, "hit-scan tree");

    sub_section(term_width, "Input");
    let input = Input::new(&mat_reg, &attrs, &tree, &grid, &sett);
    report!(input, "input");

    section(term_width, "Running");
    let data = run::multi_thread(&input).expect("Failed to run cartographer.");

    section(term_width, "Saving");
    report!(data, "data");
    data.save(&out_dir).expect("Failed to save output data.");

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
