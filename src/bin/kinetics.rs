//! MCRT programme.

use arctk::kinetics::Name;
use arctk::*;
use attr::input;
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
}

/// Main function.
pub fn main() {
    banner::title("Kinetics");

    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (sett, reactions, mut concs) = build(&in_dir, params);

    let num_steps = 100;
    let dt = 1.0 / num_steps as f64;
    for _ in 0..num_steps {
        concs = kinetics::single_thread(&sett, &reactions, concs, dt);
        for c in &concs {
            print!("{}\t", c);
        }
        println!();
    }

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
