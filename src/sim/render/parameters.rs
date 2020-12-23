//! Loadable parameters.

use crate::{
    geom::{Camera, SurfaceLinker, TreeSettings},
    img::Gradient,
    ord::Set,
    sim::render::{AttributeLinker, Engine, Settings, ShaderLinker},
};

/// Loadable runtime parameters.
pub struct Parameters {
    /// Rendering specific settings.
    pub sett: Settings,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Attributes.
    pub attrs: Set<AttributeLinker>,
    /// Colour gradients.
    pub grads: Set<Gradient>,
    /// Main camera.
    pub cam: Camera,
    /// Shader settings.
    pub shader: ShaderLinker,
    /// Engine selection.
    pub engine: Engine,
}
