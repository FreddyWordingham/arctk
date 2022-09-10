use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Verbose mode
    #[clap(short, long, value_parser)]
    verbose: bool,

    /// Input directory
    #[clap(short, long, value_parser, default_value = ".")]
    input_dir: String,

    /// Output directory
    #[clap(short, long, value_parser, default_value = "./output")]
    output_dir: String,

    /// Parameters filename
    #[clap(short, long, value_parser, default_value = "parameters.json")]
    parameters_filename: String,
}

/// Entrypoint function.
fn main() {
    let args = Args::parse();
    load_parameters(args.verbose, args.input_dir, args.parameters_filename);
    init_directories(args.verbose, args.output_dir);
}

/// Load parameters from a JSON file.
fn load_parameters(verbose: bool, input_dir: String, parameters_filename: String) {
    let parameters_filepath = input_dir + "/" + &parameters_filename;

    if verbose {
        println!("Loading parameters file: {}", parameters_filepath);
    }
}

/// Create the output directory if it does not exist.
fn init_directories(verbose: bool, output_dir: String) {
    if verbose {
        println!("Creating output directory: {}", output_dir);
    }
}
