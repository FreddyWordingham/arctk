//! Rendering engine binary.

use arctk::*;
use arctk_attr::input;
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
    /// Render runtime settings.
    sett: render::Settings,
    /// Surfaces map.
    surfs: Set<MeshBuilder>,
    /// Attributes map.
    attrs: Set<render::Attributes>,
    /// Colour map.
    cols: Set<GradientBuilder>,
    /// Shader.
    shader: render::ShaderBuilder,
}

/// Main function.
pub fn main() {
    banner::title("RENDER");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (tree_sett, render_sett, surfs, attrs, cols, shader) = build(&in_dir, params);
    let tree = grow(tree_sett, &surfs);
    let input = render::Scene::new(&tree, &render_sett, &surfs, &attrs, &cols);
    banner::section("Rendering");
    let output = render::run::multi_thread(&input, &shader).expect("Failed to perform rendering.");
    banner::section("Saving");
    output.save(&out_dir).expect("Failed to save output data.");
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
#[allow(clippy::type_complexity)]
fn build(
    in_dir: &Path,
    params: Parameters,
) -> (
    tree::Settings,
    render::Settings,
    Set<Mesh>,
    Set<render::Attributes>,
    Set<Gradient<LinSrgba>>,
    render::Shader,
) {
    banner::section("Building");
    banner::sub_section("Adaptive Tree Settings");
    let tree_sett = params.tree;
    report!("Tree settings", &tree_sett);

    banner::sub_section("Render Settings");
    let render_sett = params.sett;
    report!("Render settings", &render_sett);

    banner::sub_section("Surfaces");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    report!("Surfaces", &surfs);

    banner::sub_section("Attributes");
    let attrs = params.attrs;
    report!("Attributes", &attrs);

    banner::sub_section("Colours");
    let cols = params
        .cols
        .build(in_dir)
        .expect("Unable to build colour gradients.");
    for (group, grad) in cols.map() {
        report!(&format!("[{}]", group), gradient::to_string(&grad, 32));
    }

    banner::sub_section("Shaders");
    let shader = params
        .shader
        .build(in_dir)
        .expect("Unable to build scenes.");
    report!("Main image", &shader);

    (tree_sett, render_sett, surfs, attrs, cols, shader)
}

/// Grow domains.
fn grow<'a>(tree_sett: tree::Settings, surfs: &'a Set<Mesh>) -> tree::Cell<'a> {
    banner::section("Growing");

    banner::sub_section("Adaptive Tree");
    let tree = tree::Cell::new_root(&tree_sett, &surfs);
    report!("Adaptive tree", &tree);

    tree
}
