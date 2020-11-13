//! MCRT binary.

use arctk::{
    args,
    file::{Build, Load, Redirect, Save},
    geom::{Grid, GridBuilder, Mesh, MeshBuilder, Tree, TreeBuilder},
    ord::{Key, Set},
    sim::mcrt::{Attributes, Light, LightBuilder, Material, MaterialBuilder, Settings, Universe},
    util::{banner, dir},
};
use arctk_attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: TreeBuilder,
    /// Regular grid settings.
    grid: GridBuilder,
    /// MCRT runtime settings.
    sett: Settings,
    /// Surfaces set.
    surfs: Set<Key, MeshBuilder>,
    /// Attributes set.
    attrs: Set<Key, Attributes>,
    /// Materials set.
    mats: Set<Key, Redirect<MaterialBuilder>>,
    /// Light form.
    light: LightBuilder,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    banner::title("MCRT", term_width);

    let (params_path, in_dir, out_dir) = init(term_width);

    let params = input(term_width, &in_dir, &params_path);

    let (tree_sett, grid_sett, mcrt_sett, surfs, attrs, mats, light) =
        build(term_width, &in_dir, params);

    let (tree, grid) = grow(term_width, tree_sett, grid_sett, &surfs);

    let input = Universe::new(&tree, &grid, &mcrt_sett, &surfs, &attrs, &mats);
    let output = simulate(term_width, &input);

    banner::section("Saving", term_width);
    output.save(&out_dir).expect("Failed to save output data.");

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

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
    Set<Key, Attributes>,
    Set<Key, Material>,
    Light,
) {
    banner::section("Building", term_width);
    banner::sub_section("Adaptive Tree Settings", term_width);
    let tree_sett = params.tree;

    banner::sub_section("Grid Settings", term_width);
    let grid_sett = params.grid;

    banner::sub_section("MCRT Settings", term_width);
    let mcrt_sett = params.sett;

    banner::sub_section("Surfaces", term_width);
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Failed to build surfaces.");

    banner::sub_section("Attributes", term_width);
    let attrs = params.attrs;

    banner::sub_section("Materials", term_width);
    let mats = params
        .mats
        .build(in_dir)
        .expect("Failed to remove redirections in materials.")
        .build(in_dir)
        .expect("Failed to build materials.");

    banner::sub_section("Light", term_width);
    let light = params.light.build(in_dir).expect("Failed to build light.");

    (tree_sett, grid_sett, mcrt_sett, surfs, attrs, mats, light)
}

/// Grow domains.
fn grow(
    term_width: usize,
    tree_sett: TreeBuilder,
    grid_sett: GridBuilder,
    surfs: &Set<Key, Mesh>,
) -> (Tree<&Key>, Grid) {
    banner::section("Growing", term_width);

    banner::sub_section("Adaptive Tree", term_width);
    let tree = tree_sett.build(&surfs);

    banner::sub_section("Regular Grid", term_width);
    let grid = grid_sett.build();

    (tree, grid)
}

/// Run the simulation.
fn simulate(term_width: usize, uni: &Universe) -> Data {
    banner::section("Simulating", term_width);
    arctk::sim::mcrt::multi_thread(&uni, &light).expect("Failed to complete simulation.")
}
