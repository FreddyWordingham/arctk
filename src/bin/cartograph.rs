//! Mapping program.

use arctk::{
    args,
    file::{Build, Load, Redirect, Save},
    geom::{Grid, GridBuilder, Mesh, MeshBuilder, Tree, TreeBuilder},
    ord::{Key, Set},
    report,
    sim::cartograph::{run, Data, Engine, EngineBuilder, Interface, Landscape, Settings},
    util::{banner, dir},
};
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: Redirect<TreeBuilder>,
    /// Sampling grid settings.
    grid: Redirect<GridBuilder>,
    /// Runtime settings.
    sett: Redirect<Settings>,
    /// Surfaces set.
    surfs: Redirect<Set<Key, MeshBuilder>>,
    /// Interfaces map. (Inside material, outside material).
    inters: Redirect<Set<Key, Interface>>,
    /// Engine selection.
    engine: EngineBuilder,
}

/// Main function.
pub fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    banner::title("CARTOGRAPHER", term_width);

    let (params_path, in_dir, out_dir) = init(term_width);

    let params = input(term_width, &in_dir, &params_path);

    let (tree_sett, grid_sett, sett, surfs, inters, engine) = build(term_width, &in_dir, params);

    let (tree, grid) = grow(term_width, tree_sett, grid_sett, &surfs);

    let input = Landscape::new(&tree, &grid, &sett, &surfs, &inters);
    let output = mapping(term_width, engine, &input);

    post_analysis(term_width, &output);

    save(term_width, &out_dir, &output);

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path.display(), "binary path");
    report!(params_path.display(), "parameters path");

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    report!(in_dir.display(), "input directory");
    report!(out_dir.display(), "output directory");

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::type_complexity)]
fn build(
    term_width: usize,
    in_dir: &Path,
    params: Parameters,
) -> (
    TreeBuilder,
    GridBuilder,
    Settings,
    Set<Key, Mesh>,
    Set<Key, Interface>,
    Engine,
) {
    banner::section("Building", term_width);
    banner::sub_section("Adaptive Tree Settings", term_width);
    let tree_sett = params
        .tree
        .build(in_dir)
        .expect("Failed to redirect adaptive tree settings.");

    banner::sub_section("Measurement Grid Settings", term_width);
    let grid_sett = params
        .grid
        .build(in_dir)
        .expect("Failed to redirect grid settings.");

    banner::sub_section("Runtime Settings", term_width);
    let sett = params
        .sett
        .build(in_dir)
        .expect("Failed to redirect runtime settings.");

    banner::sub_section("Surfaces", term_width);
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Failed to redirect surfaces set.")
        .build(in_dir)
        .expect("Failed to build surfaces set.");

    banner::sub_section("Interfaces", term_width);
    let inters = params
        .inters
        .build(in_dir)
        .expect("Failed to redirect interfaces set.");

    banner::sub_section("Engine", term_width);
    let engine = params.engine.build();

    (tree_sett, grid_sett, sett, surfs, inters, engine)
}

/// Grow domains.
fn grow<'a>(
    term_width: usize,
    tree_sett: TreeBuilder,
    grid_sett: GridBuilder,
    surfs: &'a Set<Key, Mesh>,
) -> (Tree<'a, &Key>, Grid) {
    banner::section("Growing", term_width);
    banner::sub_section("Adaptive Tree", term_width);
    let tree = tree_sett.build(&surfs);

    banner::sub_section("Regular Grid", term_width);
    let grid = grid_sett.build();

    (tree, grid)
}

/// Perform the mapping.
fn mapping(term_width: usize, engine: Engine, input: &Landscape) -> Data {
    banner::section("Mapping", term_width);
    // run::single_thread(engine::basic::sample, &input).expect("Failed to complete survey.")
    run::multi_thread(engine, &input).expect("Failed to complete survey.")
}

/// Review the output data.
fn post_analysis(term_width: usize, output: &Data) {
    banner::section("Post-Analysis", term_width);

    let mut total: f64 = output.maps.map().values().map(|m| m.sum()).sum();
    report!(total, "total occupancy");

    for (key, map) in output.maps.map() {
        let occupancy = map.sum();
        report!(
            key,
            format!("({}%) {}", occupancy / total * 100.0, occupancy)
        );
        total += occupancy;
    }
}

/// Save the output data.
fn save(term_width: usize, out_dir: &Path, output: &Data) {
    banner::section("Saving", term_width);
    output.save(&out_dir).expect("Failed to save output data.");
}
