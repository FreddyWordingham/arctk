//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::{
    args,
    err::Error,
    fs::{File, Load},
    ord::Build,
    report,
    sim::babbage::Parameters,
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
    title(term_width, "Babbage");

    let (in_dir, out_dir, params_path) = initialisation(term_width);
    let params = Parameters::new_from_file(&in_dir.join(&params_path))
        .expect("Failed to load parameters file.");
    run(term_width, params, &in_dir, &out_dir).expect("Running operations failed.");

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

/// Run the operations and save their results.
fn run(term_width: usize, params: Parameters, in_dir: &Path, out_dir: &Path) -> Result<(), Error> {
    section(term_width, "Running");

    for (name, op) in params.ops {
        sub_section(term_width, &format!("Operation {}", name));
        report!(op, "operation");
        op.load(in_dir)?.build().run(&out_dir, &name)?;
    }

    Ok(())
}
