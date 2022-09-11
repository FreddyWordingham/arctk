use clap::Parser;
use std::{fs::create_dir_all, path::PathBuf};

use arctk::{parse::json, render::Parameters};

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Input directory.
    #[clap(short, long, value_parser, default_value = ".")]
    pub input_dir: PathBuf,

    /// Output directory.
    #[clap(short, long, value_parser, default_value = "output/")]
    pub output_dir: PathBuf,

    /// Parameters filename.
    #[clap(short, long, value_parser, default_value = "parameters.json")]
    pub parameters_filename: PathBuf,
}

/// Entrypoint function.
fn main() {
    let args = Args::parse();
    let _params = load_parameters(args.input_dir, args.parameters_filename);
    init_directories(args.output_dir);
}

/// Load parameters from a JSON file.
#[inline]
fn load_parameters(input_dir: PathBuf, parameters_filename: PathBuf) -> Parameters {
    let parameters_filepath = input_dir.join(&parameters_filename);
    println!("Loading parameters file: {}", parameters_filepath.display());
    json::load::<Parameters>(&parameters_filepath)
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
