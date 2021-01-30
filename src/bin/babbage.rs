//! Datacube manipulation engine binary.
//! Produce some data from some other data.

use arctk::util::{
    banner::{section, title},
    fmt::term,
};
// use std::{
//     env::current_dir,
//     path::{Path, PathBuf},
// };

/// Main program function.
fn main() {
    let term_width = term::width(80);
    title(term_width, "Babbage");

    // let (in_dir, out_dir, params_path) = initialisation(term_width);
    // let params = input(term_width, &in_dir, &params_path);
    // run(term_width, params.op, &out_dir);

    section(term_width, "Finished");
}
