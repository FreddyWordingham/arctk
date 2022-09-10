use clap::Parser;
use std::{fs::create_dir_all, path::PathBuf};

mod util;
use util::Args;

/// Entrypoint function.
fn main() {
    let args = Args::parse();
    load_parameters(args.input_dir, args.parameters_filename);
    init_directories(args.output_dir);
}

/// Load parameters from a JSON file.
#[inline]
fn load_parameters(input_dir: PathBuf, parameters_filename: PathBuf) {
    let parameters_filepath = input_dir.join(&parameters_filename);
    println!("Loading parameters file: {}", parameters_filepath.display());
}

/// Create the output directory if it does not exist.
#[inline]
fn init_directories(output_dir: PathBuf) {
    if output_dir.exists() {
        println!("Overwriting output directory: {}", output_dir.display());
    } else {
        println!("Creating output directory: {}", output_dir.display());
        create_dir_all(&output_dir)
            .unwrap_or_else(|_| panic!("Failed to create output directory!"));
    }
}
