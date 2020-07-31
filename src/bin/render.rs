//! Rendering engine binary.

use arctk::*;
use attr::input;

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: tree::Settings,
    /// Regular grid settings.
    grid: grid::Settings,
    /// Render runtime settings.
    sett: render::Settings,
    /// Surfaces map.
    surfs: Set<form::Mesh>,
    /// Attributes map.
    attrs: Set<render::Attributes>,
    /// Colour map.
    cols: Set<form::Gradient>,
    /// Scenes.
    scenes: Set<form::Scene>,
}

/// Main function.
pub fn main() {
    banner::title("RENDER");
}
