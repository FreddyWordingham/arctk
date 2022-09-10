use clap::Parser;
use std::path::PathBuf;

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
