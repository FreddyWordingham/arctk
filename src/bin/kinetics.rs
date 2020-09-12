//! MCRT programme.

use arctk::kinetics::Name;
use arctk::*;
use arctk_attr::input;
use ndarray::Array1;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Reaction simulation settings.
    sett: kinetics::Settings,
    /// Reactions.
    reactions: Vec<kinetics::ReactionBuilder>,
    /// Initial concentrations.
    concs: kinetics::ConcBuilder,
    /// Number of steps.
    num_steps: i32,
    /// Total time.
    total_time: f64,
}

/// Main function.
pub fn main() {
    banner::title("Kinetics");

    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);

    let num_steps = params.num_steps;
    let total_time = params.total_time;

    let (sett, reactions, mut concs) = build(&in_dir, params);

    // let mut file =
    //     std::fs::File::create(out_dir.join("concs.txt")).expect("Failed to create output file.");

    let mut history = Vec::with_capacity((num_steps + 1) as usize);
    history.push(concs.clone());

    let dt = total_time / num_steps as f64;
    for _ in 0..num_steps {
        concs = kinetics::single_thread(&sett, &reactions, concs, dt);
        history.push(concs.clone());
    }

    save(&out_dir, history);

    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line arguments");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!("binary path", bin_path.display());
    report!("parameters path", params_path.display());

    banner::sub_section("Directories");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let exec_name = exec::name().expect("Could not determine executable name.");

    let (in_dir, out_dir) = dir::io_dirs(
        Some(cwd.join("input").join(exec_name.clone())),
        // Some(
        //     cwd.join("output").join(exec_name).join(
        //         chrono::offset::Local::now()
        //             .format("%Y%m%d%H%M%S")
        //             .to_string(),
        //     ),
        // ),
        Some(cwd.join("output").join(exec_name)),
    )
    .expect("Could not initialise directories");
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input");
    banner::sub_section("Parameters");
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Could not load parameters file")
}

/// Build instances.
fn build(
    _in_dir: &Path,
    params: Parameters,
) -> (kinetics::Settings, Vec<kinetics::Reaction>, Array1<f64>) {
    banner::section("Building");
    banner::sub_section("Reaction Settings");
    let react_sett = params.sett;
    report!("Reaction settings", &react_sett);

    let mut names = Vec::new();
    for react in &params.reactions {
        names.append(&mut react.names());
    }
    names.sort();
    names.dedup();
    let reg = kinetics::Register::new(names);
    for i in 0..reg.names().len() {
        report!(&format!("{}", i), &reg.names()[i]);
    }

    banner::sub_section("Reactions");
    let mut reactions = Vec::with_capacity(params.reactions.len());
    for react in params.reactions {
        reactions.push(react.build(&reg).expect("Failed to build reaction."));
    }
    report!("Total reactions", reactions.len());

    banner::sub_section("Initial Concentrations");
    let concs = params
        .concs
        .build(&reg)
        .expect("Failed to build concentration array.");

    (react_sett, reactions, concs)
}

use std::{fs::File, io::Write};
fn save(out_dir: &Path, data: Vec<Array1<f64>>) {
    let mut file = File::create(out_dir.join("concs.csv")).expect("Failed to create output file.");
    for row in data {
        write!(&mut file, "{:<20}", row[0]).expect("Failed to write to file.");
        for x in row.iter().skip(1) {
            write!(&mut file, ", {:<20}", x).expect("Failed to write to file.");
        }
        writeln!(&mut file).expect("Failed to write to file.");
    }
}
