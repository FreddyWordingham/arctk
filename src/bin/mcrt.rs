use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory
    #[clap(short, long, value_parser, default_value = ".")]
    input_dir: String,

    /// Output directory
    #[clap(short, long, value_parser, default_value = "./output")]
    output_dir: String,

    /// Verbose mode
    #[clap(short, long, value_parser)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    init_directories(args.verbose, args.input_dir, args.output_dir);
}

fn init_directories(verbose: bool, input_dir: String, output_dir: String) {
    if verbose {
        println!(" input dir: {}", input_dir);
        println!("output dir: {}", output_dir);
    }
}
