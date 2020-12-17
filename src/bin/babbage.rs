//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::{
    args,
    file::{Build, Load},
    report,
    sim::babbage::{Operation, OperationBuilder},
    util::{
        banner::{section, sub_section, title},
        dir,
    },
};
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    /// Operation to perform.
    op: OperationBuilder,
}

use arctk::fmt_report;
use std::fmt::{Display, Error, Formatter};
impl Display for Parameters {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation builder");
        Ok(())
    }
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Babbage");

    let (in_dir, _out_dir, params_path) = initialisation(term_width);
    let _op = input(term_width, &in_dir, &params_path);

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

/// Retrieve the input parameters file structure.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Operation {
    section(term_width, "Input");
    sub_section(term_width, "Loading");
    let params =
        Parameters::load(&in_dir.join(&params_path)).expect("Failed to load parameters file.");
    report!(params, "parameters");

    sub_section(term_width, "Building");
    let op = params
        .op
        .build(&in_dir)
        .expect("Failed to construct Babbage operation.");
    // report!(format!("{:?}", op), "op");

    op
}
