//! MCRT programme.

use arctk::*;
use attr::input;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: tree::Settings,
    /// Regular grid settings.
    grid: grid::Settings,
    /// MCRT runtime settings.
    sett: mcrt::Settings,
    /// Surfaces set.
    surfs: Set<MeshBuilder>,
    /// Attributes set.
    attrs: Set<mcrt::Attributes>,
    /// Material set.
    mats: Set<Redirect<mcrt::MaterialBuilder>>,
    /// Light form.
    light: mcrt::LightBuilder,
}

/// Main function.
pub fn main() {
    banner::title("MCRT");

    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (tree_sett, grid_sett, mcrt_sett, surfs, attrs, mats, light) = build(&in_dir, params);
    let (tree, grid) = grow(tree_sett, grid_sett, &surfs);
    let input = mcrt::Scene::new(&tree, &grid, &mcrt_sett, &surfs, &attrs, &mats);
    banner::section("Randomising");
    let data = mcrt::run::multi_thread(&input, &light).expect("Failed to run MCRT simulation.");
    banner::section("Saving");
    data.save(&out_dir).expect("Failed to save output.");

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
    in_dir: &Path,
    params: Parameters,
) -> (
    tree::Settings,
    grid::Settings,
    mcrt::Settings,
    Set<Mesh>,
    Set<mcrt::Attributes>,
    Set<mcrt::Material>,
    mcrt::Light,
) {
    banner::section("Building");
    banner::sub_section("Adaptive Tree Settings");
    let tree_sett = params.tree;
    report!("Tree settings", &tree_sett);

    banner::sub_section("Grid Settings");
    let grid_sett = params.grid;
    report!("Grid settings", &grid_sett);

    banner::sub_section("MCRT Settings");
    let mcrt_sett = params.sett;
    report!("MCRT settings", &mcrt_sett);

    banner::sub_section("Surfaces");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    report!("Surfaces", &surfs);

    banner::sub_section("Attributes");
    let attrs = params.attrs;
    report!("Attributes", &attrs);

    banner::sub_section("Materials");
    let mats = params
        .mats
        .build(in_dir)
        .expect("Unable to build materials.")
        .build(in_dir)
        .expect("Unable to build materials.");
    report!("Properties", &mats);

    banner::sub_section("Light");
    let light = params
        .light
        .build(in_dir)
        .expect("Unable to build light source.");
    report!("Light", &light);

    (tree_sett, grid_sett, mcrt_sett, surfs, attrs, mats, light)
}

/// Grow domains.
fn grow<'a>(
    tree_sett: tree::Settings,
    grid_sett: grid::Settings,
    surfs: &'a Set<Mesh>,
) -> (tree::Cell<'a>, grid::Grid) {
    banner::section("Growing");

    banner::sub_section("Adaptive Tree");
    let tree = tree::Cell::new_root(&tree_sett, &surfs);
    report!("Adaptive tree", &tree);

    banner::sub_section("Regular Grid");
    let grid = grid::Grid::new(&grid_sett);
    report!("Regular grid", &grid);

    (tree, grid)
}
