//! Monte-Carlo radiative transfer simulation binary.
//! Compute the radiative field for a given set of setup and light source.

use arctk::{
    args,
    data::Histogram,
    fs::{File, Load, Save},
    geom::{Grid, Tree},
    ord::{Build, Link, Register, Set},
    report,
    sim::mcrt::{run, AttributeLinkerLinker, Input, Output, Parameters, ParametersBuilderLoader},
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
    title(term_width, "MCRT");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let engine = params.engine;
    report!("{* POINTER SET *}", "engine");
    let sett = params.sett;
    report!(sett, "settings");
    let grid = params.grid;
    report!(grid, "measurement grid");
    let mats = params.mats;
    report!(mats, "materials");

    sub_section(term_width, "Registration");
    let (spec_reg, img_reg) = gen_detector_registers(&params.attrs);
    let output = gen_base_output(&grid, &spec_reg, &img_reg, &params.attrs);

    sub_section(term_width, "Linking");
    let light = params
        .light
        .link(&mats)
        .expect("Failed to link materials to light.");
    report!(light, "light");
    let attrs = params
        .attrs
        .link(spec_reg.set())
        .expect("Failed to link spectrometers to attributes.")
        .link(&mats)
        .expect("Failed to link materials to attributes.");
    report!(attrs, "attributes");
    let surfs = params
        .surfs
        .link(&attrs)
        .expect("Failed to link attribute to surfaces.");
    report!(surfs, "surfaces");

    sub_section(term_width, "Growing");
    let tree = Tree::new(&params.tree, &surfs);
    report!(tree, "hit-scan tree");

    section(term_width, "Running");
    let input = Input::new(&spec_reg, &mats, &attrs, &light, &tree, &grid, &sett);
    report!(input, "input");

    let data = run::multi_thread(engine, &input, &output).expect("Failed to run cartographer.");

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

/// Generate the detector registers.
fn gen_detector_registers(attrs: &Set<AttributeLinkerLinker>) -> (Register, Register) {
    let mut spec_names = Vec::new();
    let mut img_names = Vec::new();
    for attr in attrs.map().values() {
        match *attr {
            AttributeLinkerLinker::Spectrometer(ref name, ..) => spec_names.push(name.clone()),
            AttributeLinkerLinker::Imager(ref name, ..) => img_names.push(name.clone()),
            _ => {}
        }
    }

    let spec_reg = Register::new(spec_names);
    report!(spec_reg, "spectrometer register");

    let img_reg = Register::new(img_names);
    report!(img_reg, "imager register");

    (spec_reg, img_reg)
}

/// Generate the base output instance.
fn gen_base_output<'a>(
    grid: &Grid,
    spec_reg: &'a Register,
    img_reg: &'a Register,
    attrs: &Set<AttributeLinkerLinker>,
) -> Output<'a> {
    let res = *grid.res();

    let mut specs = Vec::with_capacity(spec_reg.len());
    let mut imgs = vec![];

    for name in spec_reg.set().map().keys() {
        for attr in attrs.values() {
            match attr {
                AttributeLinkerLinker::Spectrometer(spec_name, [min, max], bins) => {
                    if name == spec_name {
                        specs.push(Histogram::new(*min, *max, *bins));
                    }
                }
                AttributeLinkerLinker::Imager(img_name, width, res, forward) => {
                    if name == img_name {
                        // imgs.push(Histogram::new(*min, *max, *bins));
                    }
                }
                _ => {}
            }
        }
    }

    Output::new(spec_reg, grid.boundary().clone(), res, specs, imgs)
}
