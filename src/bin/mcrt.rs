use clap::Parser;
use log::{debug, error, info};
use std::{fs::create_dir_all, path::PathBuf};

mod args;
use args::Args;

/// Entrypoint function.
fn main() {
    env_logger::init();
    let args = Args::parse();
    load_parameters(args.input_dir, args.parameters_filename);
    init_directories(args.output_dir);
}

/// Load parameters from a JSON file.
fn load_parameters(input_dir: PathBuf, parameters_filename: PathBuf) {
    let parameters_filepath = input_dir.join(&parameters_filename);
    info!("Loading parameters file: {}", parameters_filepath.display());
}

/// Create the output directory if it does not exist.
fn init_directories(output_dir: PathBuf) {
    if output_dir.exists() {
        debug!("Overwriting output directory: {}", output_dir.display());
    } else {
        debug!("Creating output directory: {}", output_dir.display());
        create_dir_all(&output_dir)
            .unwrap_or_else(|_| error!("Failed to create output directory!"));
    }
}
