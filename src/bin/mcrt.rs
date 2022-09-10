use clap::Parser;
use env_logger;
use log::{debug, info};
use std::{fs::create_dir_all, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory
    #[clap(short, long, value_parser, default_value = ".")]
    input_dir: PathBuf,

    /// Output directory
    #[clap(short, long, value_parser, default_value = "./output/")]
    output_dir: PathBuf,

    /// Parameters filename
    #[clap(short, long, value_parser, default_value = "parameters.json")]
    parameters_filename: PathBuf,
}

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
        create_dir_all(&output_dir).expect("Failed to create output directory!");
    }
}
