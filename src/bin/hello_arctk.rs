//! Hello Arctk!

use arctk::*;
use std::path::PathBuf;

/// Main function.
pub fn main() {
    banner::title("HELLO ARCTK");
    init();
}

/// Initialise the command line arguments and directories.
fn init() {
    banner::section("Initialisation");
    banner::sub_section("Command line arguments");
    args!(bin_path: PathBuf;
        name: String
    );
    println!("Hello {}!", name);
}
