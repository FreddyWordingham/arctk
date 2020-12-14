//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::{
    args,
    file::{Build, Load, Save},
    geom::Tree,
    ord::Link,
    sim::babbage::OperationBuilder,
    util::{
        banner::{section, title},
        dir,
    },
};
use arctk_attr::input;
use std::{env::current_dir, path::PathBuf};

// Input parameters.
#[input]
struct Parameters {
    /// Operation to perform.
    op: OperationBuilder,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Babbage");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Failed to load parameters file.");

    section(term_width, "Building");
    let op = params
        .op
        .build(&in_dir)
        .expect("Failed to construct Babbage operation.");

    section(term_width, "Operation");
    op.run(&out_dir)
        .expect("Operation failed... we'll get 'em next time.");

    section(term_width, "Finished");
}
