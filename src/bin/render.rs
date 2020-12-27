//! Rendering engine binary.
//! Produce two-dimensional image data from a three-dimensional scene.

use arctk::{
    args,
    fs::{File, Load, Save},
    geom::Tree,
    ord::{Build, Link},
    report,
    sim::render::{run, Input, Parameters, ParametersBuilderLoader},
    util::{
        banner::{section, sub_section, title},
        dir,
        fmt::gradient::to_string,
    },
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Main render function.
fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Render");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = load_parameters(term_width, &in_dir, &params_path);

    section(term_width, "Input");
    sub_section(term_width, "Reconstruction");
    let engine = params.engine;
    report!("{* POINTER SET *}", "engine");
    let sett = params.sett;
    report!(sett, "settings");
    let grads = params.grads;
    for (key, val) in grads.map() {
        report!(to_string(val, 32), &format!("{}", key));
    }
    let cam = params.cam;
    report!(cam, "camera");

    sub_section(term_width, "Linking");
    let attrs = params
        .attrs
        .link(&grads)
        .expect("Failed to link x to attributes.");
    report!(attrs, "attributes");
    let surfs = params
        .surfs
        .link(&attrs)
        .expect("Failed to link attribute to surfaces.");
    report!(surfs, "surfaces");
    let shader = params
        .shader
        .link(&grads)
        .expect("Failed to link attribute to shader.");
    report!(shader, "shader");

    sub_section(term_width, "Growing");
    let tree = Tree::new(&params.tree, &surfs);
    report!(tree, "hist-scan tree");

    sub_section(term_width, "Input");
    let input = Input::new(&grads, &attrs, &cam, &tree, &sett, &shader);
    report!(input, "input");

    section(term_width, "Running");
    let data = run::multi_thread(engine, &input).expect("Failed to run cartographer.");

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
