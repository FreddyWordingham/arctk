//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{Camera, GridBuilder, SurfaceLinker, TreeSettings},
    img::Gradient,
    ord::Set,
    sim::render::{AttributeLinker, Settings, ShaderLinker},
};
use arctk_attr::file;
use std::path::Path;

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
