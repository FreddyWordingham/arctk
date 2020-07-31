//! Arctk rendering engine.

use arctk::*;
use attr::input;
use palette::{Gradient, LinSrgba};
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
    /// Render runtime settings.
    sett: render::Technical,
    /// Surfaces map.
    surfs: Set<form::Mesh>,
    /// Attributes map.
    attrs: Set<render::Attributes>,
    /// Colour map.
    cols: Set<form::Gradient>,
    /// Scenes.
    scenes: Set<form::Scene>,
}

// /// Main function.
pub fn main() {
    banner::title("RENDER");

    //     let (params_path, in_dir, out_dir) = init();

    //     let params = input(&in_dir, &params_path);

    //     let (tree_sett, grid_sett, render_sett, surfs, attrs, cols, scenes) = build(&in_dir, params);
    //     let engine = render_sett.engine();

    //     let (tree, grid) = grow(tree_sett, grid_sett, &surfs);

    //     let input = render::Input::new(&tree, &grid, &render_sett, &surfs, &attrs, &cols);

    //     for (name, scene) in scenes.map() {
    //         banner::section(&format!("Scene: {}", name));

    //         match engine {
    //             form::Engine::Green => {
    //                 let output = render::engine::green::fast::run(&input, &scene);
    //                 output
    //                     .expect("Rendering failed.")
    //                     .save(&out_dir)
    //                     .expect("Failed to save output data.");
    //             }
    //             form::Engine::Antler => {
    //                 let output = render::engine::antler::fast::run(&input, &scene);
    //                 output
    //                     .expect("Rendering failed.")
    //                     .save(&out_dir)
    //                     .expect("Failed to save output data.");
    //             }
    //         };
    //     }

    banner::section("Finished");
}

// /// Initialise the command line arguments and directories.
// fn init() -> (PathBuf, PathBuf, PathBuf) {
//     banner::section("Initialisation");
//     banner::sub_section("Command line arguments");
//     args!(bin_path: PathBuf;
//         params_path: PathBuf
//     );
//     report!("binary path", bin_path.display());
//     report!("parameters path", params_path.display());

//     banner::sub_section("Directories");
//     let cwd = current_dir().expect("Failed to determine current working directory.");
//     let exec_name = exec::name().expect("Could not determine executable name.");
//     let (in_dir, out_dir) = dir::io_dirs(
//         Some(cwd.join("input").join(exec_name.clone())),
//         Some(cwd.join("output").join(exec_name)),
//     )
//     .expect("Could not initialise directories");
//     report!("input directory", in_dir.display());
//     report!("output directory", out_dir.display());

//     (params_path, in_dir, out_dir)
// }

// /// Load the input files.
// fn input(in_dir: &Path, params_path: &Path) -> Parameters {
//     banner::section("Input");
//     banner::sub_section("Parameters");
//     let path = in_dir.join(params_path);

//     Parameters::load(&path).expect("Could not load parameters file")
// }

// /// Build instances.
// fn build(
//     in_dir: &Path,
//     params: Parameters,
// ) -> (
//     tree::Settings,
//     grid::Settings,
//     render::Technical,
//     Set<Mesh>,
//     Set<render::Attributes>,
//     Set<Gradient<LinSrgba>>,
//     Set<render::Scene>,
// ) {
//     banner::section("Building");
//     banner::sub_section("Adaptive Tree Settings");
//     let tree_sett = params.tree;
//     report!("Tree settings", &tree_sett);

//     banner::sub_section("Grid Settings");
//     let grid_sett = params.grid;
//     report!("Grid settings", &grid_sett);

//     banner::sub_section("Render Settings");
//     let render_sett = params.sett;
//     report!("Render settings", &render_sett);

//     banner::sub_section("Surfaces");
//     let surfs = params
//         .surfs
//         .build(in_dir)
//         .expect("Unable to build surfaces.");
//     report!("Surfaces", &surfs);

//     banner::sub_section("Attributes");
//     let attrs = params.attrs;
//     report!("Attributes", &attrs);

//     banner::sub_section("Colours");
//     let cols = params
//         .cols
//         .build(in_dir)
//         .expect("Unable to build colour gradients.");
//     for (group, grad) in cols.map() {
//         report!(&format!("[{}]", group), gradient::to_string(&grad, 32));
//     }

//     banner::sub_section("Scenes");
//     let scenes = params
//         .scenes
//         .build(in_dir)
//         .expect("Unable to build scenes.");
//     report!("Scenes", &scenes);

//     (
//         tree_sett,
//         grid_sett,
//         render_sett,
//         surfs,
//         attrs,
//         cols,
//         scenes,
//     )
// }

// /// Grow domains.
// fn grow<'a>(
//     tree_sett: tree::Settings,
//     grid_sett: grid::Settings,
//     surfs: &'a Set<Mesh>,
// ) -> (tree::Cell<'a>, grid::Grid) {
//     banner::section("Growing");

//     banner::sub_section("Adaptive Tree");
//     let tree = tree::Cell::new_root(&tree_sett, &surfs);
//     report!("Adaptive tree", &tree);

//     banner::sub_section("Regular Grid");
//     let grid = grid::Grid::new(&grid_sett);
//     report!("Regular grid", &grid);

//     (tree, grid)
// }
